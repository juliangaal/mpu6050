//! All constants used in the driver, mostly register addresses
//! Register map: https://arduino.ua/docs/RM-MPU-6000A.pdf
//! Datasheet with WAY more info about interrupts (Revision 3.2) https://www.cdiweb.com/datasheets/invensense/ps-mpu-6000a.pdf
//!
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
    //
}

/// Slave address of Mpu6050
pub const SLAVE_ADDR: u8 = 0x68;
/// Internal register to check slave addr
pub const WHOAMI: u8 = 0x75;
/// Motion Threshold Register
pub const MOT_THR: u8 = 0x1F;
/// Motion Duration Detection Register
pub const MOT_DUR: u8 = 0x20;

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
/// Register 26: Configuration (DLPF, External signal)
pub struct CONFIG;

impl CONFIG {
    /// Base Address
    pub const ADDR: u8 = 0x1a;
    /// external Frame Synchronisation (FSYNC)
    pub const EXT_SYNC_SET: BitBlock = BitBlock { bit: 5, length: 3};
    /// Digital Low Pass Filter (DLPF) config
    pub const DLPF_CFG: BitBlock = BitBlock { bit: 2, length: 3};
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
/// Register 55: INT Pin / Bypass Enable Configuration
pub struct INT_PIN_CFG;

impl INT_PIN_CFG {
    /// Base Address
    pub const ADDR: u8 = 0x37;
    /// INT pin logic level
    pub const INT_LEVEL: u8 = 7;
    /// INT pin config
    pub const INT_OPEN: u8 = 6;
    /// Pulse (length)
    pub const LATCH_INT_EN: u8 = 5;
    /// INT clear conditions
    pub const INT_RD_CLEAR: u8 = 4;
    /// FSYNC PIN logic level
    pub const FSYNC_INT_LEVEL: u8 = 3;
    /// FSYNC PIN config
    pub const FSYNC_INT_EN: u8 = 2;
    /// i2c access/bypass
    pub const I2C_BYPASS_EN: u8 = 1;
    /// enable/disable reference clock output
    pub const CLKOUT_EN: u8 = 0;
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
/// Register 56: Interrupt Status
pub struct INT_ENABLE;

impl INT_ENABLE {
    /// Base Address
    pub const ADDR: u8 = 0x38;
    /// Generate interrupt Free Fall Detection
    pub const FF_EN: u8 = 7;
    /// Generate interrupt with Motion Detected
    pub const MOT_EN: u8 = 6;
    /// Generate iterrrupt when Zero Motion Detection
    pub const ZMOT_EN: u8 = 5;
    /// Generate iterrupt when FIFO buffer overflow
    pub const FIFO_OFLOW_END: u8 = 4;
    /// this  bit enables  any  of  the  I2C  Masterinterrupt  sources  to generate an interrupt
    pub const I2C_MST_INT_EN: u8 = 3;
    /// enables Data Ready interrupt, each time a write operation to all sensor registers completed
    pub const DATA_RDY_EN: u8 = 0;
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
/// Register 58: Interrupt Status
pub struct INT_STATUS;

impl INT_STATUS {
    /// Base Address
    pub const ADDR: u8 = 0x3a;
    /// Free Fall Interrupt
    pub const FF_INT: u8 = 7;
    /// Motion Detection Interrupt
    pub const MOT_INT: u8 = 6;
    /// Zero Motion Detection Interrupt
    pub const ZMOT_INT: u8 = 5;
    /// FIFO buffer overflow
    pub const FIFO_OFLOW_INT: u8 = 4;
    /// i2c master interrupt has been generated
    pub const I2C_MSF_INT: u8 = 3;
    /// Data is ready
    pub const DATA_RDY_INT: u8 = 0;
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
/// Register 97: Motion Detection Status
pub struct MOT_DETECT_STATUS;

impl MOT_DETECT_STATUS {
    /// Base Address
    pub const ADDR: u8 = 0x61;
    /// motion  in  the  negative  X  axis  has generated a Motion detection interrupt
    pub const MOT_XNEG: u8 = 7;
    /// motion  in  the  positive  X  axis  has generated a Motion detection interrupt
    pub const MOT_XPOS: u8 = 6;
    /// motion  in  the  negative  Y  axis  has generated a Motion detection interrupt
    pub const MOT_YNEG: u8 = 5;
    /// motion  in  the positive  Y  axis  has generated a Motion detection interrupt
    pub const MOT_YPOS: u8 = 4;
    /// motion  in  the  negative  Z  axis  has generated a Motion detection interrupt.
    pub const MOT_ZNEG: u8 = 3;
    /// motion  in  the  positive  Z  axis  has generated a Motion detection interrupt
    pub const MOT_ZPOS: u8 = 2;
    /// Zero  Motion detection  interrupt  is generated
    pub const MOT_ZRMOT: u8 = 0;
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
/// Register 105: Motion Detection Control
pub struct MOT_DETECT_CONTROL;

impl MOT_DETECT_CONTROL {
    /// Base Address
    pub const ADDR: u8 = 0x69;
    /// Additional delay
    pub const ACCEL_ON_DELAY: BitBlock = BitBlock { bit: 5, length: 2};
    ///  Free Fall count
    pub const FF_COUNT: BitBlock = BitBlock { bit: 3, length: 2};
    /// Motion Detection cound
    pub const MOT_COUNT: BitBlock = BitBlock { bit: 1, length: 2};
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
/// Register 107: Power Management 1
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
#[derive(Copy, Clone, Debug)]
/// Register 107: Power Management 2
pub struct PWR_MGMT_2;

impl PWR_MGMT_2 {
    /// Base Address
    pub const ADDR: u8 = 0x6c;
    /// Wake up frequency
    pub const LP_WAKE_CTRL: BitBlock = BitBlock { bit: 7, length: 2};
    /// disable accel axis x
    pub const STBY_XA: u8 = 5;
    /// disable accel axis y
    pub const STBY_YA: u8 = 4;
    /// disable accel axis z
    pub const STBY_ZA: u8 = 3;
    /// disable gyro  axis x
    pub const STBY_XG: u8 = 2;
    /// disable gyro  axis y
    pub const STBY_YG: u8 = 1;
    /// disable gyro  axis z
    pub const STBY_ZG: u8 = 0;
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// Wake values
pub enum LP_WAKE_CTRL {
    _1P25 = 0,
    _2P5,
    _5,
    _10,
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