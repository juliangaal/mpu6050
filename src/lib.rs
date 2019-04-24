#![no_std]

pub mod registers;

///! Mpu6050 sensor driver.
///! Register sheet: https://www.invensense.com/wp-content/uploads/2015/02/MPU-6000-Register-Map1.pdf
///! Data sheet: https://www.invensense.com/wp-content/uploads/2015/02/MPU-6500-Datasheet2.pdf 

use crate::registers::*;
use libm::{powf, atan2f, sqrtf};
use embedded_hal::{
    blocking::delay::DelayMs,
    blocking::i2c::{Write, WriteRead},
};

/// Operations trait for sensor readings
pub trait MutOps {
    /// Add values to each readings fields
    fn add(&mut self, operand: &Self);
    /// Scales object fields with foo * 1/n
    fn scale(&mut self, n: u8);
}

/// Used for bias calculation of chip in mpu::soft_calib
#[derive(Default, Debug, Clone)]
pub struct Bias {
    /// accelerometer x axis bias
    ax: f32,
    /// accelerometer y axis bias
    ay: f32,
    /// accelerometer z axis bias
    az: f32,
    /// gyro x axis bias
    gx: f32,
    /// gyro y axis bias
    gy: f32,
    /// gyro z axis bias
    gz: f32,
    /// temperature AVERAGE: can't get bias!
    t: f32,
}

impl Bias {
    fn add(&mut self, acc: RotReading, gyro: RotReading, temp: f32) {
        self.ax += acc.x;
        self.ay += acc.y;
        self.az += acc.z;
        self.gx += gyro.x;
        self.gy += gyro.y;
        self.gz += gyro.z;
        self.t += temp; 
    }

    fn scale(&mut self, n: u8) {
        let n = n as f32;
        self.ax /= n;
        self.ay /= n;
        self.az /= n;
        self.gx /= n;
        self.gy /= n;
        self.gz /= n;
        self.t /= n;
    }
}

pub type Variance = Bias;

impl Variance {
    fn add_diff(&mut self, acc_diff: (f32, f32, f32), gyro_diff: (f32, f32, f32), temp_diff: f32) {
        self.ax += acc_diff.0;
        self.ay += acc_diff.1;
        self.az += acc_diff.2;
        self.gx += gyro_diff.0;
        self.gy += gyro_diff.1;
        self.gz += gyro_diff.2;
        self.t += temp_diff; 
    }
}

/// Struct for rotation reading: gyro or accelerometer 
#[derive(Debug)]
pub struct RotReading {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl RotReading {
    fn new(x: f32, y: f32, z: f32) -> Self {
        RotReading {
            x, 
            y, 
            z,
        }
    }
}

impl MutOps for RotReading {
    fn add(&mut self, operand: &Self) {
        self.x += operand.x;
        self.y += operand.y;
        self.z += operand.z;
    }

    fn scale(&mut self, n: u8) {
        let n = n as f32;
        self.x /= n;
        self.y /= n;
        self.z /= n;
    }
}

/// struct for Roll/Pitch Reading
#[derive(Debug)]
pub struct RPReading {
    pub roll: f32,
    pub pitch: f32,
}

impl RPReading {
    fn new(roll: f32, pitch: f32) -> Self {
        RPReading {
            roll,
            pitch,
        }
    }
}

impl MutOps for RPReading {
    fn add(&mut self, operand: &Self) {
        self.roll += operand.roll;
        self.pitch += operand.pitch;
    }

    fn scale(&mut self, n: u8) {
        let n = n as f32;
        self.roll /= n;
        self.pitch /= n;
    }
}

/// Helper struct used as number of steps for filtering
pub struct Steps(pub u8);
pub type Mask = Steps;

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
pub enum Mpu6050Error<E> {
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
    variance: Option<Variance>,
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
            variance: None,
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
            variance: None,
            acc_sensitivity: Sensitivity::from(arange).0,
            gyro_sensitivity: Sensitivity::from(grange).0,
        }
    }

    /// Wakes MPU6050 with all sensors enabled (default)
    pub fn wake(&mut self) -> Result<(), Mpu6050Error<E>> {
        self.write_u8(POWER_MGMT_1, 0)?;
        self.delay.delay_ms(100u8);
        Ok(())
    }

    /// Init wakes MPU6050 and verifies register addr, e.g. in i2c
    pub fn init(&mut self) -> Result<(), Mpu6050Error<E>> {
        self.wake()?;
        self.verify()?;
        Ok(())
    }

    /// Verifies device to address 0x68 with WHOAMI Register
    pub fn verify(&mut self) -> Result<(), Mpu6050Error<E>> {
        let address = self.read_u8(WHOAMI)?;
        if address != SLAVE_ADDR {
            return Err(Mpu6050Error::InvalidChipId(address));
        }
        Ok(())
    }

    /// Performs software calibration with steps number of readings
    /// of accelerometer and gyrometer sensor
    /// Readings must be made with MPU6050 in resting position
    pub fn soft_calib(&mut self, steps: Steps) -> Result<(), Mpu6050Error<E>> {
        let mut bias = Bias::default();

        for _ in 0..steps.0+1 {
            bias.add(self.get_acc()?, self.get_gyro()?, self.get_temp()?);
        }   

        bias.scale(steps.0);
        bias.az -= 1.0; // gravity compensation
        self.bias = Some(bias);

        Ok(())
    }
    
    /// Get bias of measurements
    pub fn get_bias(&mut self) -> Option<&Bias> {
        self.bias.as_ref()
    }

    /// Get variance of sensor by observing in resting state for steps 
    /// number of readings: accelerometer, gyro and temperature sensor each
    pub fn calc_variance(&mut self, steps: Steps) -> Result<(), Mpu6050Error<E>> {
        let iterations = steps.0;
        if let None = self.bias {
            self.soft_calib(steps)?;
        }
        
        let mut variance = Variance::default();
        let mut acc = self.get_acc()?;
        let mut gyro = self.get_gyro()?;
        let mut temp = self.get_temp()?;
        let mut acc_diff: (f32, f32, f32); 
        let mut gyro_diff: (f32, f32, f32);
        let mut temp_diff: f32;
        let bias = self.bias.clone().unwrap();

        for _ in 0..iterations {
           acc_diff = (powf(acc.x - bias.ax, 2.0), powf(acc.y - bias.ay, 2.0), powf(acc.z - bias.az, 2.0)); 
           gyro_diff = (powf(gyro.x - bias.gx, 2.0), powf(gyro.y - bias.gy, 2.0), powf(gyro.z - bias.gz, 2.0));
           temp_diff = powf(temp - bias.t, 2.0);
           variance.add_diff(acc_diff, gyro_diff, temp_diff);
           acc = self.get_acc()?;
           gyro = self.get_gyro()?;
           temp = self.get_temp()?;
        }

        variance.scale(iterations-1);
        variance.az -= 1.0; // gravity compensation
        self.variance = Some(variance);

        Ok(())
    }

    /// get variance of measurements
    pub fn get_variance(&mut self) -> Option<&Variance> {
        self.variance.as_ref()
    }

    /// Roll and pitch estimation from raw accelerometer readings
    /// NOTE: no yaw! no magnetometer present on MPU6050
    pub fn get_acc_angles(&mut self) -> Result<RPReading, Mpu6050Error<E>> {
        let acc = self.get_acc()?;
        let roll: f32 = atan2f(acc.y, sqrtf(powf(acc.x, 2.) + powf(acc.z, 2.)));
        let pitch: f32 = atan2f(-acc.x, sqrtf(powf(acc.y, 2.) + powf(acc.z, 2.)));
        Ok(RPReading::new(roll, pitch))
    }

    /// Roll and pitch estimation from raw accelerometer - averaged across window readings
    pub fn get_acc_angles_avg(&mut self, mask: Mask) -> Result<RPReading, Mpu6050Error<E>> {
        let mut acc = self.get_acc_angles()?;
        for _ in 0..mask.0-1 {
            acc.add(&self.get_acc_angles()?);
        }
        acc.scale(mask.0);
        Ok(acc)
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
    fn read_rot(&mut self, reg: u8) -> Result<RotReading, Mpu6050Error<E>> {
        let mut buf: [u8; 6] = [0; 6];
        self.read_bytes(reg, &mut buf)?;

        let xr = self.read_word_2c(&buf[0..2]);
        let yr = self.read_word_2c(&buf[2..4]);
        let zr = self.read_word_2c(&buf[4..6]);
        
        Ok(RotReading::new(xr as f32, yr as f32, zr as f32)) // returning as f32 makes future calculations easier
    }

    /// Accelerometer readings in m/s^2
    pub fn get_acc(&mut self) -> Result<RotReading, Mpu6050Error<E>> {
        let mut acc = self.read_rot(ACC_REGX_H)?;
        
        acc.x /= self.acc_sensitivity;
        acc.y /= self.acc_sensitivity;
        acc.z /= self.acc_sensitivity;

        if let Some(ref bias) = self.bias { 
            acc.x -= bias.ax;
            acc.y -= bias.ay;
            acc.z -= bias.az;
        }

        Ok(acc)
    }

    /// Accelerometer readings in m/s^2 - averaged
    pub fn get_acc_avg(&mut self, mask: Mask) -> Result<RotReading, Mpu6050Error<E>> {
        let mut acc = self.get_acc()?;
        for _ in 0..mask.0-1 {
            acc.add(&self.get_acc()?);
        }
        acc.scale(mask.0);
        Ok(acc)
    }

    /// Gyro readings in rad/s
    pub fn get_gyro(&mut self) -> Result<RotReading, Mpu6050Error<E>> {
        let mut gyro = self.read_rot(GYRO_REGX_H)?;

        gyro.x *= PI / (180.0 * self.gyro_sensitivity);
        gyro.y *= PI / (180.0 * self.gyro_sensitivity);
        gyro.z *= PI / (180.0 * self.gyro_sensitivity);

        if let Some(ref bias) = self.bias {
            gyro.x -= bias.gx;
            gyro.y -= bias.gy;
            gyro.z -= bias.gz;
        }

        Ok(gyro)
    }
    
    /// Gyro readings in rad/s
    pub fn get_gyro_avg(&mut self, mask: Mask) -> Result<RotReading, Mpu6050Error<E>> {
        let mut gyro = self.get_gyro()?;
        for _ in 0..mask.0-1 {
            gyro.add(&self.get_gyro()?);
        }
        gyro.scale(mask.0);
        Ok(gyro)
    }

    /// Temp in degrees celcius
    pub fn get_temp(&mut self) -> Result<f32, Mpu6050Error<E>> {
        let mut buf: [u8; 2] = [0; 2];
        self.read_bytes(TEMP_OUT_H, &mut buf)?;
        let raw_temp = self.read_word_2c(&buf[0..2]) as f32;

        Ok((raw_temp / 340.) + 36.53)
    } 
    
    pub fn get_temp_avg(&mut self, mask: Mask) -> Result<f32, Mpu6050Error<E>> {
        let mut temp = self.get_temp()?;
        for _ in 0..mask.0-1 {
            temp += self.get_temp()?;
        }
        temp /= mask.0 as f32;
        Ok(temp)
    }

    /// Writes byte to register
    pub fn write_u8(&mut self, reg: u8, byte: u8) -> Result<(), Mpu6050Error<E>> {
        self.i2c.write(SLAVE_ADDR, &[reg, byte])
            .map_err(Mpu6050Error::I2c)?;
        self.delay.delay_ms(10u8);
        Ok(())
    }
    
    /// Reads byte from register
    pub fn read_u8(&mut self, reg: u8) -> Result<u8, Mpu6050Error<E>> {
        let mut byte: [u8; 1] = [0; 1];
        self.i2c.write_read(SLAVE_ADDR, &[reg], &mut byte)
            .map_err(Mpu6050Error::I2c)?;
        Ok(byte[0])
    }

    /// Reads series of bytes into buf from specified reg
    pub fn read_bytes(&mut self, reg: u8, buf: &mut [u8]) -> Result<(), Mpu6050Error<E>> {
        self.i2c.write_read(SLAVE_ADDR, &[reg], buf)
            .map_err(Mpu6050Error::I2c)?;
        Ok(())
    }
}
