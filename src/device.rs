//! All constants used in the driver, mostly register addresses
//! Register map: https://arduino.ua/docs/RM-MPU-6000A.pdf
//! Datasheet with WAY more info about interrupts (Revision 3.2) https://www.cdiweb.com/datasheets/invensense/ps-mpu-6000a.pdf
//!
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub enum Registers {
    /// Slave address of Mpu6050
    SLAVE_ADDR = 0x68,
    /// Internal register to check slave addr
    WHOAMI = 0x75,
    /// High Byte Register Gyro x orientation
    GYRO_REGX_H = 0x43,
    /// High Byte Register Gyro y orientation
    GYRO_REGY_H = 0x45,
    /// High Byte Register Gyro z orientation
    GYRO_REGZ_H = 0x47,
    /// High Byte Register Calc roll
    ACC_REGX_H = 0x3b,
    /// High Byte Register Calc pitch
    ACC_REGY_H = 0x3d,
    /// High Byte Register Calc yaw
    ACC_REGZ_H = 0x3f,
    /// High Byte Register Temperature
    TEMP_OUT_H = 0x41,
    /// Register to control chip waking from sleep, enabling sensors, default: sleep
    POWER_MGMT_1 = 0x6b, 
    /// Internal clock
    POWER_MGMT_2 = 0x6c, 
    /// Accelerometer config register
    ACCEL_CONFIG  = 0x1c,
    /// gyro config register
    GYRO_CONFIG = 0x1b,
}

impl Registers {
    pub fn addr(&self) -> u8 {
        *self as u8
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub struct Bits;


impl Bits {
    /// Accelerometer high pass filter bit: See 4.5 Register 28
    pub const ACCEL_HPF_BIT: u8 = 3;
    pub const GYRO_CONFIG_FS_SEL_BIT: u8 = 4;
    pub const GYRO_CONFIG_FS_SEL_LENGTH: u8 = 3;
}