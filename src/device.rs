//! All constants used in the driver, mostly register addresses
//! Register map: https://arduino.ua/docs/RM-MPU-6000A.pdf
//! Datasheet with WAY more info about interrupts (Revision 3.2) https://www.cdiweb.com/datasheets/invensense/ps-mpu-6000a.pdf
//!

/// Gyro Sensitivity
pub const GYRO_SENS: (f32, f32, f32, f32) = (131., 65.5, 32.8, 16.4);

/// Accelerometer Sensitivity
pub const ACCEL_SENS: (f32, f32, f32, f32) = (16384., 8192., 4096., 2048.);

/// Temperature Offset
pub const TEMP_OFFSET: f32 = 36.53;

/// Temperature Sensitivity
pub const TEMP_SENSITIVITY: f32 = 340.;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
/// Register addresses
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

    /// Gyro Config FS_SEL start bit
    pub const GYRO_CONFIG_FS_SEL_BIT: u8 = 4;
    /// Gyro Config FS_SEL length
    pub const GYRO_CONFIG_FS_SEL_LENGTH: u8 = 3;


    /// Accel Config FS_SEL start bit
    pub const ACCEL_CONFIG_FS_SEL_BIT: u8 = 4;
    /// Accel Config FS_SEL length
    pub const ACCEL_CONFIG_FS_SEL_LENGTH: u8 = 2;
    /// Accel Config FS_SEL start bit
    pub const ACCEL_CONFIG_ACCEL_HPF_BIT: u8 = 2;
    /// Accel Config FS_SEL length
    pub const ACCEL_CONFIG_ACCEL_HPF_LENGTH: u8 = 3;
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ACCEL_HPF {
    _RESET = 0,
    _5 = 1,
    _2P5 = 2,
    _1P25 = 3,
    _0P63 = 4,
    _HOLD = 7
}

impl From<u8> for ACCEL_HPF {
    fn from(range: u8) -> Self
    {
        match range {
            0 => ACCEL_HPF::_RESET,
            1 => ACCEL_HPF::_5,
            2 => ACCEL_HPF::_2P5,
            3 => ACCEL_HPF::_1P25,
            4 => ACCEL_HPF::_0P63,
            7 => ACCEL_HPF::_HOLD,
            _ => ACCEL_HPF::_RESET,
        }
    }
}
//
// #[derive(Copy, Clone, Debug)]
// pub struct BitBlock {
//     reg: u8,
//     start_bit: u8,
//     length: u8
// }
//
// pub const ACONFIG_ACCEL_HBF: BitBlock = BitBlock { reg: Registers::ACCEL_CONFIG.addr(), start_bit: Bits::ACCEL_CONFIG_ACCEL_HBF_BIT, length: Bits::ACCEL_CONFIG_ACCEL_HBF_LENGTH};