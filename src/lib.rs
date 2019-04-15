#![no_std]

///! Mpu6050 sensor driver.
///! Datasheet: 
use embedded_hal::{
    blocking::delay::DelayMs,
    blocking::i2c::{Read, Write, WriteRead},
};

const MPU6050_SLAVE_ADDR: u8 = 0x68;
const MPU6050_WHOAMI: u8 = 0x75;

/// High Bytle Register Gyro x orientation
const MPU6050_GYRO_REGX_H: u8 = 0x43;
/// High Bytle Register Gyro y orientation
const MPU6050_GYRO_REGY_H: u8 = 0x45;
/// High Bytle Register Gyro z orientation
const MPU6050_GYRO_REGZ_H: u8 = 0x47;
 
/// High Byte Register Calc roll
const MPU6050_ACC_REGX_H: u8 = 0x3b;
/// High Byte Register Calc pitch
const MPU6050_ACC_REGY_H: u8 = 0x3d;
/// High Byte Register Calc yaw
const MPU6050_ACC_REGZ_H: u8 = 0x3f;

/// Register to control chip waking from sleep, enabling sensors, default: sleep
const POWER_MGMT_1: u8 = 0x6b; 
/// Internal clock
const POWER_MGMT_2: u8 = 0x6c; 

/// Gyro Sensitivity
const FS_SEL: (f32, f32, f32, f32) = (131., 65.5, 32.8, 16.4);
/// Calcelerometer Sensitivity
const AFS_SEL: (f32, f32, f32, f32) = (16384., 8192., 4096., 2048.);

const PI: f32 = 3.14159;

const GRAVITIY_MS2: f32 = 9.80665;

const ACCEL_RANGE_2G: u8 = 0x00;
const ACCEL_RANGE_4G: u8 = 0x08;
const ACCEL_RANGE_8G: u8 = 0x10;
const ACCEL_RANGE_16G: u8 = 0x18;

const GYRO_RANGE_250DEG: u8 = 0x00;
const GYRO_RANGE_500DEG: u8 = 0x08;
const GYRO_RANGE_1000DEG: u8 = 0x10;
const GYRO_RANGE_2000DEG: u8 = 0x18;

const ACCEL_CONFIG: u8  = 0x1c;
const GYRO_CONFIG: u8 = 0x1b;

enum AccelRange {
    G2,
    G4,
    G8,
    G16,
}

enum GyroRange {
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

pub struct Mpu6050<I, D> {
    i2c: I,
    delay: D
}

impl<I, D, E> Mpu6050<I, D>
where
    I: Write<Error = E> + WriteRead<Error = E>,
    D: DelayMs<u8>, 
{
    pub fn new(i2c: I, delay: D) -> Self {
        Mpu6050 {
            i2c,
            delay,
        }
    }

    pub fn soft_calibration(steps: u8, ) -> Result<(), Error<E>> {
        Ok(())
    }

    pub fn wake(&mut self) -> Result<(), Error<E>> {
        self.write_u8(POWER_MGMT_1, 0)?;
        self.delay.delay_ms(100u8);
        Ok(())
    }

    pub fn init(&mut self) -> Result<(), Error<E>> {
        self.wake()?;
        self.verify()?;
        Ok(())
    }

    pub fn verify(&mut self) -> Result<(), Error<E>> {
        let address = self.read_u8(MPU6050_WHOAMI)?;
        if address != MPU6050_SLAVE_ADDR {
            return Err(Error::InvalidChipId(address));
        }
        Ok(())
    }

    /// Implements complementary filter for roll/pitch
    /// NOTE: yaw will always point up, sensor has no magnetometer to allow fusion
    pub fn rpy(&mut self) -> Result<(f32, f32, f32), Error<E>> {
        Ok((0.0, 0.0, 0.0))
    }

    // TODO work on removing unnecessary type conversion
    fn read_word_2c(&self, byte: &[u8]) -> i32 {
        let high: i32 = byte[0] as i32;
        let low: i32 = byte[1] as i32;
        let mut word: i32 = (high << 8) + low;

        if word >= 0x8000 {
            word = -((65535 - word) + 1);
        }

        word
    }

    fn read_rot(&mut self, reg: u8, scaling: f32) -> Result<(f32, f32, f32), Error<E>> {
        let mut buf: [u8; 6] = [0; 6];
        let bytes = self.read_bytes(reg, &mut buf)?;

        let xr = self.read_word_2c(&buf[0..2]);
        let yr = self.read_word_2c(&buf[2..4]);
        let zr = self.read_word_2c(&buf[4..6]);
        
        Ok((xr as f32, yr as f32, zr as f32)) // returning as f32 makes future calculations easier
    }

    /// Accelerometer readings in m/s^2
    pub fn get_acc(&mut self) -> Result<(f32, f32, f32), Error<E>> {
        let mut buf: [u8; 6] = [0; 6];
        let (mut ax, mut ay, mut az) = self.read_rot(MPU6050_ACC_REGX_H, AFS_SEL.0)?;
        
        ax /= AFS_SEL.0;
        ay /= AFS_SEL.0;
        az /= AFS_SEL.0;

        Ok((ax, ay, az))
    }

    /// Gyro readings in rad/s
    pub fn get_gyro(&mut self) -> Result<(f32, f32, f32), Error<E>> {
        let mut buf: [u8; 6] = [0; 6];
        let (mut gx, mut gy, mut gz) = self.read_rot(MPU6050_GYRO_REGX_H, FS_SEL.0)?;

        if gy >= 0x8000 as f32 {
            panic!("Shit");
        }
        
        gx *= PI / (180.0 * FS_SEL.0);
        gy *= PI / (180.0 * FS_SEL.0);
        gz *= PI / (180.0 * FS_SEL.0);

        Ok((gx, gy, gz))
    }

    pub fn write_u8(&mut self, reg: u8, byte: u8) -> Result<(), Error<E>> {
        self.i2c.write(MPU6050_SLAVE_ADDR, &[reg, byte])
            .map_err(Error::I2c)?;
        self.delay.delay_ms(10u8);
        Ok(())
    }

    pub fn read_u8(&mut self, reg: u8) -> Result<u8, Error<E>> {
        let mut byte: [u8; 1] = [0; 1];
        self.i2c.write_read(MPU6050_SLAVE_ADDR, &[reg], &mut byte)
            .map_err(Error::I2c)?;
        Ok(byte[0])
    }

    pub fn read_bytes(&mut self, reg: u8, buf: &mut [u8]) -> Result<(), Error<E>> {
        self.i2c.write_read(MPU6050_SLAVE_ADDR, &[reg], buf)
            .map_err(Error::I2c)?;
        Ok(())
    }
}
