//! Mpu6050 sensor driver.
//!
//! Register sheet [here](https://www.invensense.com/wp-content/uploads/2015/02/MPU-6000-Register-Map1.pdf),
//! Data sheet [here](https://www.invensense.com/wp-content/uploads/2015/02/MPU-6500-Datasheet2.pdf)
//! 
//! To use this driver you must provide a concrete `embedded_hal` implementation.//! This example uses `linux_embedded_hal`
//! ```
//! let i2c = I2cdev::new("/dev/i2c-1")
//!     .map_err(Mpu6050Error::I2c)?;
//!
//! let delay = Delay;
//!
//! let mut mpu = Mpu6050::new(i2c, delay);
//! mpu.init()?;
//! mpu.soft_calib(Steps(100))?;
//! mpu.calc_variance(Steps(50))?;
//!
//! println!("Calibrated with bias: {:?}", mpu.get_bias().unwrap());
//! println!("Calculated variance: {:?}", mpu.get_variance().unwrap());
//!
//! // get roll and pitch estimate
//! let acc = mpu.get_acc_angles()?;
//!
//! // get roll and pitch estimate - averaged accross n readings (steps)
//! let acc = mpu.get_acc_angles_avg(Steps(5))?;
//!
//! // get temp
//! let temp = mpu.get_temp()?;
//!
//! // get temp - averages across n readings (steps)
//! let temp = mpu.get_temp_avg(Steps(5))?;
//!
//! // get gyro data, scaled with sensitivity 
//! let gyro = mpu.get_gyro()?;
//! 
//! // get gyro data, scaled with sensitivity - averaged across n readings (steps) 
//! let gyro = mpu.get_gyro_avg(Steps(5))?;
//! 
//! // get accelerometer data, scaled with sensitivity
//! let acc = mpu.get_acc()?;
//! 
//! // get accelerometer data, scaled with sensitivity - averages across n readings (steps)
//! let acc = mpu.get_acc_avg(Steps(5))?;
//! ```

#![no_std]

pub mod registers;

use crate::registers::Registers::*;
use libm::{powf, atan2f, sqrtf};
use nalgebra::{Vector3, Vector2};
use embedded_hal::{
    blocking::delay::DelayMs,
    blocking::i2c::{Write, WriteRead},
};

/// pi, taken straight from C
pub const PI: f32 = 3.14159265358979323846;
/// Gyro Sensitivity
pub const FS_SEL: (f32, f32, f32, f32) = (131., 65.5, 32.8, 16.4);
/// Calcelerometer Sensitivity
pub const AFS_SEL: (f32, f32, f32, f32) = (16384., 8192., 4096., 2048.);


/// Operations trait for sensor readings
pub trait MutOps {
    /// Add values to each readings fields
    fn add(&mut self, operand: &Self);
    /// Scales object fields with foo * 1/n
    fn scale(&mut self, n: u8);
}

/// Helpers for sensor readings
pub trait Access<T> {
    fn x(&self) -> T;
    fn set_x(&mut self, val: T);
    fn y(&self) -> T;
    fn set_y(&mut self, val: T);
    fn z(&self) -> T;
    fn set_z(&mut self, val: T);
}

/// Trait for conversion from/to radians/degree
pub trait UnitConv<T> {
    /// get radians from degree
    fn to_rad(&self) -> T;
    /// get radians from degree and change underlying data
    fn to_rad_mut(&mut self);
    /// get degree from radians
    fn to_deg(&self) -> T;
    /// get degree from radians and change underlying data
    fn to_deg_mut(&mut self);
}

impl UnitConv<f32> for f32 {
    fn to_rad(&self) -> f32 {
        self * PI/180.0
    } 
    
    fn to_rad_mut(&mut self) {
        *self *= PI/180.0
    }

    fn to_deg(&self) -> f32 {
        self * 180.0/PI
    }   

    fn to_deg_mut(&mut self) {
        *self *= 180.0/PI
    }   
}

impl Access<f32> for Vector3<f32> {
    fn x(&self) -> f32 {
        self[0]
    }

    fn set_x(&mut self, val: f32) {
        self[0] = val;
    }

    fn y(&self) -> f32 {
        self[1]
    }

    fn set_y(&mut self, val: f32) {
        self[1] = val;
    }

    fn z(&self) -> f32 {
        self[2]
    }

    fn set_z(&mut self, val: f32) {
        self[2] = val
    }
}

impl Access<f32> for Vector2<f32> {
    fn x(&self) -> f32 {
        self[0]
    }

    fn set_x(&mut self, val: f32) {
        self[0] = val;
    }

    fn y(&self) -> f32 {
        self[1]
    }

    fn set_y(&mut self, val: f32) {
        self[1] = val;
    }

    fn z(&self) -> f32 { 
        -1.0
    }

    fn set_z(&mut self, _val: f32) {}
}

/// Used for bias calculation of chip in mpu::soft_calib
#[derive(Debug, Clone)]
pub struct Bias {
    /// accelerometer axis bias
    acc: Vector3<f32>,
    /// gyro x axis bias
    gyro: Vector3<f32>,
    /// temperature AVERAGE: can't get bias!
    temp: f32,
}

impl Default for Bias {
    fn default() -> Bias {
        Bias {
            acc: Vector3::<f32>::zeros(),
            gyro: Vector3::<f32>::zeros(),
            temp: 0.0,
        }
    }
}

impl Bias {
    fn add(&mut self, acc: Vector3<f32>, gyro: Vector3<f32>, temp: f32) {
        self.acc += acc;
        self.gyro += gyro;
        self.temp += temp; 
    }

    fn scale(&mut self, n: u8) {
        let n = n as f32;
        self.acc /= n;
        self.gyro /= n;
        self.temp /= n;
    }
}

/// Reuse Bias struct for Variance calculation
pub type Variance = Bias;

impl Variance {
    fn add_diff(&mut self, acc_diff: Vector3<f32>, gyro_diff: Vector3<f32>, temp_diff: f32) {
        self.acc += acc_diff;
        self.gyro += gyro_diff;
        self.temp += temp_diff; 
    }
}

/// Vector2 for Roll/Pitch Reading
impl UnitConv<Vector2<f32>> for Vector2<f32> {
    fn to_rad(&self) -> Vector2<f32> {
        Vector2::<f32>::new(
            self.x().to_rad(),
            self.y().to_rad(),
        )
    }

    fn to_rad_mut(&mut self) {
        self[0].to_rad_mut();
        self[1].to_rad_mut();
    }

    fn to_deg(&self) -> Vector2<f32> {
        Vector2::<f32>::new(
            self.x().to_deg(),
            self.y().to_deg(),
        )
    }

    fn to_deg_mut(&mut self) {
        self[0].to_deg_mut();
        self[1].to_deg_mut();
    }
}

/// Helper struct used as number of steps for filtering
pub struct Steps(pub u8);

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
        self.write_u8(POWER_MGMT_1.addr(), 0)?;
        self.delay.delay_ms(100u8);
        Ok(())
    }

    /// Init wakes MPU6050 and verifies register addr, e.g. in i2c
    pub fn init(&mut self) -> Result<(), Mpu6050Error<E>> {
        self.wake()?;
        self.verify()?;
        Ok(())
    }

    /// Verifies device to address 0x68 with WHOAMI.addr() Register
    pub fn verify(&mut self) -> Result<(), Mpu6050Error<E>> {
        let address = self.read_u8(WHOAMI.addr())?;
        if address != SLAVE_ADDR.addr() {
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
        bias.acc[2] -= 1.0; // gravity compensation
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
        let mut acc_diff = Vector3::<f32>::zeros(); 
        let mut gyro_diff = Vector3::<f32>::zeros();
        let mut temp_diff: f32;
        let bias = self.bias.clone().unwrap();

        for _ in 0..iterations {
           acc_diff.set_x(powf(acc.x() - bias.acc.x(), 2.0));
           acc_diff.set_y(powf(acc.y() - bias.acc.y(), 2.0));
           acc_diff.set_z(powf(acc.z() - bias.acc.z(), 2.0)); 
           gyro_diff.set_x(powf(gyro.x() - bias.gyro.x(), 2.0));
           gyro_diff.set_y(powf(gyro.y() - bias.gyro.y(), 2.0));
           gyro_diff.set_z(powf(gyro.z() - bias.gyro.z(), 2.0));
           temp_diff = powf(temp - bias.temp, 2.0);
           variance.add_diff(acc_diff, gyro_diff, temp_diff);
           acc = self.get_acc()?;
           gyro = self.get_gyro()?;
           temp = self.get_temp()?;
        }

        variance.scale(iterations-1);
        variance.acc[2] -= 1.0; // gravity compensation
        self.variance = Some(variance);

        Ok(())
    }

    /// get variance of measurements
    pub fn get_variance(&mut self) -> Option<&Variance> {
        self.variance.as_ref()
    }

    /// Roll and pitch estimation from raw accelerometer readings
    /// NOTE: no yaw! no magnetometer present on MPU6050
    pub fn get_acc_angles(&mut self) -> Result<Vector2<f32>, Mpu6050Error<E>> {
        let acc = self.get_acc()?;
        Ok(Vector2::<f32>::new(
            atan2f(acc.y(), sqrtf(powf(acc.x(), 2.) + powf(acc.z(), 2.))),
            atan2f(-acc.x(), sqrtf(powf(acc.y(), 2.) + powf(acc.z(), 2.)))
        ))
    }

    /// Roll and pitch estimation from raw accelerometer - averaged across window readings
    pub fn get_acc_angles_avg(&mut self, steps: Steps) -> Result<Vector2<f32>, Mpu6050Error<E>> {
        let mut acc = self.get_acc_angles()?;
        for _ in 0..steps.0-1 {
            acc += self.get_acc_angles()?;
        }
        acc /= steps.0 as f32;
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
    fn read_rot(&mut self, reg: u8) -> Result<Vector3<f32>, Mpu6050Error<E>> {
        let mut buf: [u8; 6] = [0; 6];
        self.read_bytes(reg, &mut buf)?;

        Ok(Vector3::<f32>::new(
            self.read_word_2c(&buf[0..2]) as f32,
            self.read_word_2c(&buf[2..4]) as f32,
            self.read_word_2c(&buf[4..6]) as f32
        ))
    }

    /// Accelerometer readings in m/s^2
    pub fn get_acc(&mut self) -> Result<Vector3<f32>, Mpu6050Error<E>> {
        let mut acc = self.read_rot(ACC_REGX_H.addr())?;
        
        acc /= self.acc_sensitivity;

        if let Some(ref bias) = self.bias { 
            acc -= bias.acc;
        }

        Ok(acc)
    }

    /// Accelerometer readings in m/s^2 - averaged
    pub fn get_acc_avg(&mut self, steps: Steps) -> Result<Vector3<f32>, Mpu6050Error<E>> {
        let mut acc = self.get_acc()?;
        for _ in 0..steps.0-1 {
            acc += self.get_acc()?;
        }
        acc /= steps.0 as f32;
        Ok(acc)
    }

    /// Gyro readings in rad/s
    pub fn get_gyro(&mut self) -> Result<Vector3<f32>, Mpu6050Error<E>> {
        let mut gyro = self.read_rot(GYRO_REGX_H.addr())?;

        gyro *= PI / (180.0 * self.gyro_sensitivity);

        if let Some(ref bias) = self.bias {
            gyro -= bias.gyro;
        }

        Ok(gyro)
    }
    
    /// Gyro readings in rad/s
    pub fn get_gyro_avg(&mut self, steps: Steps) -> Result<Vector3<f32>, Mpu6050Error<E>> {
        let mut gyro = self.get_gyro()?;
        for _ in 0..steps.0-1 {
            gyro += self.get_gyro()?;
        }

        gyro /= steps.0 as f32;
        Ok(gyro)
    }

    /// Temp in degrees celcius
    pub fn get_temp(&mut self) -> Result<f32, Mpu6050Error<E>> {
        let mut buf: [u8; 2] = [0; 2];
        self.read_bytes(TEMP_OUT_H.addr(), &mut buf)?;
        let raw_temp = self.read_word_2c(&buf[0..2]) as f32;

        Ok((raw_temp / 340.) + 36.53)
    } 
    
    pub fn get_temp_avg(&mut self, steps: Steps) -> Result<f32, Mpu6050Error<E>> {
        let mut temp = self.get_temp()?;
        for _ in 0..steps.0-1 {
            temp += self.get_temp()?;
        }
        temp /= steps.0 as f32;
        Ok(temp)
    }

    /// Writes byte to register
    pub fn write_u8(&mut self, reg: u8, byte: u8) -> Result<(), Mpu6050Error<E>> {
        self.i2c.write(SLAVE_ADDR.addr(), &[reg, byte])
            .map_err(Mpu6050Error::I2c)?;
        self.delay.delay_ms(10u8);
        Ok(())
    }
    
    /// Reads byte from register
    pub fn read_u8(&mut self, reg: u8) -> Result<u8, Mpu6050Error<E>> {
        let mut byte: [u8; 1] = [0; 1];
        self.i2c.write_read(SLAVE_ADDR.addr(), &[reg], &mut byte)
            .map_err(Mpu6050Error::I2c)?;
        Ok(byte[0])
    }

    /// Reads series of bytes into buf from specified reg
    pub fn read_bytes(&mut self, reg: u8, buf: &mut [u8]) -> Result<(), Mpu6050Error<E>> {
        self.i2c.write_read(SLAVE_ADDR.addr(), &[reg], buf)
            .map_err(Mpu6050Error::I2c)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra as na;

    #[test]
    fn test_unit_conv() {
        assert_eq!(1.0.to_rad(), 0.01745329252);

        let mut deg = 1.0;
        deg.to_rad_mut();
        assert_eq!(deg, 0.01745329252);

        assert_eq!(1.0.to_deg(), 57.295776);

        let mut rad = 1.0;
        rad.to_deg_mut();
        assert_eq!(rad, 57.295776);
    }

    #[test]
    fn test_nalgebra() {
        let mut v = Vector3::<f32>::new(1., 1., 1.);
        let o = v.clone();
        v *= 3.;
        assert_eq!(Vector3::<f32>::new(3., 3., 3.), v);
        v /= 3.;
        assert_eq!(o, v);
        v -= o;
        assert_eq!(Vector3::<f32>::new(0., 0., 0.), v);
    }
}
