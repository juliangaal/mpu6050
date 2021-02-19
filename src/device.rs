//! All constants used in the driver, mostly register addresses
//! Register map: https://arduino.ua/docs/RM-MPU-6000A.pdf
//! Datasheet with WAY more info about interrupts (Revision 3.2) https://www.cdiweb.com/datasheets/invensense/ps-mpu-6000a.pdf
//!

/// Gyro Sensitivity
///
/// Measurements are scaled like this:
/// x * range/2**(resolution-1) or x / (2**(resolution-1) / range)
/// Sources:
///     * https://www.nxp.com/docs/en/application-note/AN3461.pdf
///     * https://theccontinuum.com/2012/09/24/arduino-imu-pitch-roll-from-accelerometer/
///     * https://makersportal.com/blog/2019/8/17/arduino-mpu6050-high-frequency-accelerometer-and-gyroscope-data-saver#accel_test
///     * https://github.com/kriswiner/MPU6050/wiki/2014-Invensense-Developer%27s-Conference
///     * rust MPU9250 driver on github
pub const GYRO_SENS: (f32, f32, f32, f32) = (131., 65.5, 32.8, 16.4);

/// Accelerometer Sensitivity
///
/// Measurements are scaled like this:
/// x * range/2**(resolution-1) or x / (2**(resolution-1) / range)
/// Sources:
///     * https://www.nxp.com/docs/en/application-note/AN3461.pdf
///     * https://theccontinuum.com/2012/09/24/arduino-imu-pitch-roll-from-accelerometer/
///     * https://makersportal.com/blog/2019/8/17/arduino-mpu6050-high-frequency-accelerometer-and-gyroscope-data-saver#accel_test
///     * https://github.com/kriswiner/MPU6050/wiki/2014-Invensense-Developer%27s-Conference
///     * rust MPU9250 driver on github
pub const ACCEL_SENS: (f32, f32, f32, f32) = (16384., 8192., 4096., 2048.);

/// Temperature Offset
pub const TEMP_OFFSET: f32 = 36.53;

/// Temperature Sensitivity
pub const TEMP_SENSITIVITY: f32 = 340.;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub struct Specs;

impl Specs {
    // pub const ACCEL_SELF_TEST_MIN: u8 = -14;
    pub const ACCEL_SELF_TEST_MAX: u8 = 14;
}

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
    /// Internal clock
    POWER_MGMT_2 = 0x6c,
}

/// Describes a bit block from bit number 'bit' to 'bit'+'length'
pub struct BitBlock {
    pub bit: u8,
    pub length: u8
}

impl Registers {
    pub fn addr(&self) -> u8 {
        *self as u8
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
/// Register 27: Gyro Config
pub struct GYRO_CONFIG;

impl GYRO_CONFIG {
    pub const ADDR: u8 = 0x1b;
    /// Gyro x axis self test bit
    pub const XG_ST: u8 = 7;
    /// Gyro y axis self test bit
    pub const YG_ST: u8 = 6;
    /// Gyro z axis self test bit
    pub const ZG_ST: u8 = 5;
    /// Gyro Config FS_SEL
    pub const FS_SEL: BitBlock = BitBlock { bit: 4, length: 2 };
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
/// Register 28: Accel Config
pub struct ACCEL_CONFIG;

impl ACCEL_CONFIG {
    /// Base Address
    pub const ADDR: u8 = 0x1c;
    /// Accel x axis self test bit
    pub const XA_ST: u8 = 7;
    /// Accel y axis self test bit
    pub const YA_ST: u8 = 6;
    /// Accel z axis self test bit
    pub const ZA_ST: u8 = 5;
    /// Accel Config FS_SEL
    pub const FS_SEL: BitBlock = BitBlock { bit: 4, length: 2};
    /// Accel Config ACCEL_HPF
    pub const ACCEL_HPF: BitBlock = BitBlock { bit: 2, length: 3};
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
/// Register 107: Power Management
pub struct PWR_MGMT_1;

impl PWR_MGMT_1 {
    /// Base Address
    pub const ADDR: u8 = 0x6b;
    /// Device Reset bit
    pub const DEVICE_RESET: u8 = 7;
    /// Sleep mode bit (Should be called "Low Power", doesn't actually sleep)
    pub const SLEEP: u8 = 6;
    /// Cycle bit for wake operations
    pub const CYCLE: u8 = 5;
    /// Temperature sensor enable/disable bit
    pub const TEMP_DIS: u8 = 3;
    /// Clock Control
    pub const CLKSEL: BitBlock = BitBlock { bit: 2, length: 3 };
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// Accelerometer High Pass Filter Values
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

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// Clock Source Select Values
pub enum CLKSEL {
    OSCILL = 0,
    GXAXIS = 1,
    GYAXIS = 2,
    GZAXIS = 3,
    EXT_32p7 = 4,
    EXT_19P2 = 5,
    RESERV = 6,
    STOP = 7,
}

impl From<u8> for CLKSEL {
    fn from(clk: u8) -> Self {
        match clk {
            0 => CLKSEL::OSCILL,
            1 => CLKSEL::GXAXIS,
            2 => CLKSEL::GYAXIS,
            3 => CLKSEL::GZAXIS,
            4 => CLKSEL::EXT_32p7,
            5 => CLKSEL::EXT_19P2,
            6 => CLKSEL::RESERV,
            7 => CLKSEL::STOP,
            _ => CLKSEL::GXAXIS
        }
    }
}