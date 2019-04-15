#![no_std]

pub mod constants;

///! Mpu6050 sensor driver.
///! Register sheet: https://www.invensense.com/wp-content/uploads/2015/02/MPU-6000-Register-Map1.pdf
///! Data sheet: https://www.invensense.com/wp-content/uploads/2015/02/MPU-6500-Datasheet2.pdf 

use crate::constants::*;
use libm::{powf, atan2f, sqrtf};
use embedded_hal::{
    blocking::delay::DelayMs,
    blocking::i2c::{Write, WriteRead},
};

/// Used for bias calculation of chip in mpu::soft_calib
#[derive(Default)]
struct Bias {
    ax: f32,
    ay: f32,
    az: f32,
    gx: f32,
    gy: f32,
    gz: f32,
}

impl Bias {
    fn add(&mut self, acc: (f32, f32, f32), gyro: (f32, f32, f32)) {
        self.ax += acc.0;
        self.ay += acc.1;
        self.az += acc.2;
        self.gx += gyro.0;
        self.gy += gyro.1;
        self.gz += gyro.2;
    }

    fn scale(&mut self, n: u8) {
        let n = n as f32;
        self.ax /= n;
        self.ay /= n;
        self.az /= n;
        self.gx /= n;
        self.gy /= n;
        self.gz /= n;
    }
}

// Helper struct to convert Sensor measurement range to appropriate values defined in datasheet
struct Sensitivity(f32);

// Converts accelerometer range to correction/scaling factor, see table p. 29 or register sheet
impl From<AccelRange> for Sensitivity {
    fn from(range: AccelRange) -> Sensitivity {
        match range {
            AccelRange::G2 => return Sensitivity(AFS_SEL.0),
            AccelRange::G4 => return Sensitivity(AFS_SEL.1),
            AccelRange::G8 => return Sensitivity(AFS_SEL.2),
            AccelRange::G16 => return Sensitivity(AFS_SEL.3),
        }
    }
}

// Converts gyro range to correction/scaling factor, see table p. 31 or register sheet
impl From<GyroRange> for Sensitivity {
    fn from(range: GyroRange) -> Sensitivity {
        match range {
            GyroRange::DEG250 => return Sensitivity(FS_SEL.0),
            GyroRange::DEG500 => return Sensitivity(FS_SEL.1),
            GyroRange::DEG1000 => return Sensitivity(FS_SEL.2),
            GyroRange::DEG2000 => return Sensitivity(FS_SEL.3),
        }
    }
}

/// Defines accelerometer range/sensivity
pub enum AccelRange {
    G2,
    G4,
    G8,
    G16,
}

/// Defines gyro range/sensitivity
pub enum GyroRange {
    DEG250,
    DEG500,
    DEG1000,
    DEG2000,
}

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I2C bus error
    I2c(E),

    /// Invalid chip ID was read
    InvalidChipId(u8),
}

/// Handles all operations on/with Mpu6050
pub struct Mpu6050<I, D> {
    i2c: I,
    delay: D,
    bias: Option<Bias>,
    acc_sensitivity: f32,
    gyro_sensitivity: f32,
}

impl<I, D, E> Mpu6050<I, D>
where
    I: Write<Error = E> + WriteRead<Error = E>,
    D: DelayMs<u8>, 
{
    /// Side effect free constructor with default sensitivies, no calibration
    pub fn new(i2c: I, delay: D) -> Self {
        Mpu6050 {
            i2c,
            delay,
            bias: None,
            acc_sensitivity: AFS_SEL.0,
            gyro_sensitivity: FS_SEL.0, 
        }
    }

    /// custom sensitivity
    pub fn new_with_sens(i2c: I, delay: D, arange: AccelRange, grange: GyroRange) -> Self {
        Mpu6050 {
            i2c,
            delay,
            bias: None,
            acc_sensitivity: Sensitivity::from(arange).0,
            gyro_sensitivity: Sensitivity::from(grange).0,
        }
    }

    /// Performs software calibration with steps number of readings.
    /// Readings must be made with MPU6050 in resting position
    pub fn soft_calib(&mut self, steps: u8) -> Result<(), Error<E>> {
        let mut bias = Bias::default();

        for _ in 0..steps+1 {
            bias.add(self.get_acc()?, self.get_gyro()?);
        }   

        bias.scale(steps);
        self.bias = Some(bias);

        Ok(())
    }

    /// Wakes MPU6050 with all sensors enabled (default)
    pub fn wake(&mut self) -> Result<(), Error<E>> {
        self.write_u8(POWER_MGMT_1, 0)?;
        self.delay.delay_ms(100u8);
        Ok(())
    }

    /// Init wakes MPU6050 and verifies register addr, e.g. in i2c
    pub fn init(&mut self) -> Result<(), Error<E>> {
        self.wake()?;
        self.verify()?;
        Ok(())
    }

    /// Verifies device to address 0x68 with WHOAMI Register
    pub fn verify(&mut self) -> Result<(), Error<E>> {
        let address = self.read_u8(WHOAMI)?;
        if address != SLAVE_ADDR {
            return Err(Error::InvalidChipId(address));
        }
        Ok(())
    }

    /// Roll and pitch estimation from raw accelerometer readings
    /// NOTE: no yaw! no magnetometer present on MPU6050
    pub fn get_acc_angles(&mut self) -> Result<(f32, f32), Error<E>> {
        let (ax, ay, az) = self.get_acc()?;
        let roll: f32 = atan2f(ay, sqrtf(powf(ax, 2.) + powf(az, 2.)));
        let pitch: f32 = atan2f(-ax, sqrtf(powf(ay, 2.) + powf(az, 2.)));
        Ok((roll, pitch))
    }

    /// Converts 2 bytes number in 2 compliment
    /// TODO i16?! whats 0x8000?!
    fn read_word_2c(&self, byte: &[u8]) -> i32 {
        let high: i32 = byte[0] as i32;
        let low: i32 = byte[1] as i32;
        let mut word: i32 = (high << 8) + low;

        if word >= 0x8000 {
            word = -((65535 - word) + 1);
        }

        word
    }

    /// Reads rotation (gyro/acc) from specified register
    fn read_rot(&mut self, reg: u8) -> Result<(f32, f32, f32), Error<E>> {
        let mut buf: [u8; 6] = [0; 6];
        self.read_bytes(reg, &mut buf)?;

        let xr = self.read_word_2c(&buf[0..2]);
        let yr = self.read_word_2c(&buf[2..4]);
        let zr = self.read_word_2c(&buf[4..6]);
        
        Ok((xr as f32, yr as f32, zr as f32)) // returning as f32 makes future calculations easier
    }

    /// Accelerometer readings in m/s^2
    pub fn get_acc(&mut self) -> Result<(f32, f32, f32), Error<E>> {
        let (mut ax, mut ay, mut az) = self.read_rot(ACC_REGX_H)?;
        
        ax /= self.acc_sensitivity;
        ay /= self.acc_sensitivity;
        az /= self.acc_sensitivity;

        if let Some(ref bias) = self.bias { 
            ax -= bias.ax;
            ay -= bias.ay;
            az -= bias.az;
        }

        Ok((ax, ay, az))
    }

    /// Gyro readings in rad/s
    pub fn get_gyro(&mut self) -> Result<(f32, f32, f32), Error<E>> {
        let (mut gx, mut gy, mut gz) = self.read_rot(GYRO_REGX_H)?;

        gx *= PI / (180.0 * self.gyro_sensitivity);
        gy *= PI / (180.0 * self.gyro_sensitivity);
        gz *= PI / (180.0 * self.gyro_sensitivity);

        if let Some(ref bias) = self.bias {
            gx -= bias.gx;
            gy -= bias.gy;
            gz -= bias.gz;
        }

        Ok((gx, gy, gz))
    }

    /// Temp in degrees celcius
    pub fn get_temp(&mut self) -> Result<f32, Error<E>> {
        let mut buf: [u8; 2] = [0; 2];
        self.read_bytes(TEMP_OUT_H, &mut buf)?;
        let raw_temp = self.read_word_2c(&buf[0..2]) as f32;

        Ok((raw_temp / 340.) + 36.53)
    }

    /// Writes byte to register
    pub fn write_u8(&mut self, reg: u8, byte: u8) -> Result<(), Error<E>> {
        self.i2c.write(SLAVE_ADDR, &[reg, byte])
            .map_err(Error::I2c)?;
        self.delay.delay_ms(10u8);
        Ok(())
    }
    
    /// Reads byte from register
    pub fn read_u8(&mut self, reg: u8) -> Result<u8, Error<E>> {
        let mut byte: [u8; 1] = [0; 1];
        self.i2c.write_read(SLAVE_ADDR, &[reg], &mut byte)
            .map_err(Error::I2c)?;
        Ok(byte[0])
    }

    /// Reads series of bytes into buf from specified reg
    pub fn read_bytes(&mut self, reg: u8, buf: &mut [u8]) -> Result<(), Error<E>> {
        self.i2c.write_read(SLAVE_ADDR, &[reg], buf)
            .map_err(Error::I2c)?;
        Ok(())
    }
}
