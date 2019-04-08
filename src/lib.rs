//#![no_std]

///! Mpu6050 sensor driver.
///! Datasheet: 
use embedded_hal::{
    blocking::delay::DelayMs,
    blocking::i2c::{Read, Write, WriteRead},
};

const Mpu6050_SLAVE_ADDR: u8 = 0x68;
const Mpu6050_WHOAMI: u8 = 0x75;

/// High Bytle Register Gyro x orientation
const Mpu6050_GYRO_REGX_H: u8 = 0x43;
/// High Bytle Register Gyro y orientation
const Mpu6050_GYRO_REGY_H: u8 = 0x45;
/// High Bytle Register Gyro z orientation
const Mpu6050_GYRO_REGZ_H: u8 = 0x47;
 
/// High Byte Register Acc roll
const Mpu6050_ACC_REGX_H: u8 = 0x3b;
/// High Byte Register Acc pitch
const Mpu6050_ACC_REGY_H: u8 = 0x3d;
/// High Byte Register Acc yaw
const Mpu6050_ACC_REGZ_H: u8 = 0x3f;

/// Register to control chip waking from sleep, enabling sensors, default: sleep
const POWER_MGMT_1: u8 = 0x6b; 
/// Internal clock
const POWER_MGMT_2: u8 = 0x6c; 

/// Gyro Sensitivity
const FS_SEL: (f64, f64, f64, f64) = (131., 65.5, 32.8, 16.4);
/// Accelerometer Sensitivity
const AFS_SEL: (f64, f64, f64, f64) = (16384., 8192., 4096., 2048.);

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I2C bus error
    I2c(E),

    /// Invalid chip ID was read
    InvalidChipId(u8),
}

pub struct Rotation<T> {
    x: T,
    y: T,
    z: T,
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

    pub fn init(&mut self) -> Result<(), Error<E>> {
        self.write(POWER_MGMT_1, 0)?;

        self.verify()?;

        self.delay.delay_ms(100u8);
        Ok(())
    }

    pub fn verify(&mut self) -> Result<(), Error<E>> {
        let address = self.read(Mpu6050_WHOAMI)?;
        if address != Mpu6050_SLAVE_ADDR {
            return Err(Error::InvalidChipId(address));
        }

        println!("address {}", address);
        Ok(())
    }

    pub fn gyro_data(&mut self) -> Result<Rotation<f32>, Error<E>> {
        Ok(Rotation { x: 1., y: 2., z: 3. })
    }

    pub fn write(&mut self, address: u8, byte: u8) -> Result<(), Error<E>> {
        self.i2c.write(Mpu6050_SLAVE_ADDR, &[address, byte])
            .map_err(Error::I2c)?;
        
        self.delay.delay_ms(10u8);
        Ok(())
    }

    pub fn read(&mut self, address: u8) -> Result<u8, Error<E>> {
        let mut buffer = [0];
        self.i2c.write_read(Mpu6050_SLAVE_ADDR, &[address], &mut buffer)
            .map_err(Error::I2c)?;

        println!("GOT {:?}", &buffer);
        Ok(buffer[0])
    }
}
