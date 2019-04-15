#![no_std]

pub mod constants;

///! Mpu6050 sensor driver.
///! Datasheet: 
use embedded_hal::{
    blocking::delay::DelayMs,
    blocking::i2c::{Read, Write, WriteRead},
};

use crate::constants::*;

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
    delay: D,
    bias: Option<Bias>,
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
            bias: None,
        }
    }

    pub fn soft_calibrate(&mut self, steps: u8) -> Result<(), Error<E>> {
        let mut bias = Bias::default();

        for _ in 0..steps+1 {
            bias.add(self.get_acc()?, self.get_gyro()?);
        }   

        bias.scale(steps);
        self.bias = Some(bias);

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
        let (mut ax, mut ay, mut az) = self.read_rot(MPU6050_ACC_REGX_H, AFS_SEL.0)?;
        
        ax /= AFS_SEL.0;
        ay /= AFS_SEL.0;
        az /= AFS_SEL.0;

        if let Some(ref bias) = self.bias { 
            ax -= bias.ax;
            ay -= bias.ay;
            az -= bias.az;
        }

        Ok((ax, ay, az))
    }

    /// Gyro readings in rad/s
    pub fn get_gyro(&mut self) -> Result<(f32, f32, f32), Error<E>> {
        let (mut gx, mut gy, mut gz) = self.read_rot(MPU6050_GYRO_REGX_H, FS_SEL.0)?;

        gx *= PI / (180.0 * FS_SEL.0);
        gy *= PI / (180.0 * FS_SEL.0);
        gz *= PI / (180.0 * FS_SEL.0);

        if let Some(ref bias) = self.bias {
            gx -= bias.gx;
            gy -= bias.gy;
            gz -= bias.gz;
        }

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
