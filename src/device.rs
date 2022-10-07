//! All device constants used in the driver, mostly register addresses.
//!
//! NOTE: Earlier revisions of Datasheet and Register Map has way more info about interrupt usage,
//! particularly rev 3.2
//!
//! #### Sources:
//! * Register map (rev 3.2): https://arduino.ua/docs/RM-MPU-6000A.pdf
//! * Datasheet (rev 3.2): https://www.cdiweb.com/datasheets/invensense/ps-mpu-6000a.pdf

/// Gyro Sensitivity
///
/// Measurements are scaled like this:
/// x * range/2**(resolution-1) or x / (2**(resolution-1) / range)
///
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
///
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

/// Motion Threshold Register
pub const MOT_THR: u8 = 0x1F;
/// Motion Duration Detection Register
pub const MOT_DUR: u8 = 0x20;
/// High Byte Register Gyro x orientation
pub const GYRO_REGX_H: u8 = 0x43;
/// High Byte Register Gyro y orientation
pub const GYRO_REGY_H: u8 = 0x45;
/// High Byte Register Gyro z orientation
pub const GYRO_REGZ_H: u8 = 0x47;
/// High Byte Register Calc roll
pub const ACC_REGX_H: u8 = 0x3b;
/// High Byte Register Calc pitch
pub const ACC_REGY_H: u8 = 0x3d;
/// High Byte Register Calc yaw
pub const ACC_REGZ_H: u8 = 0x3f;
/// High Byte Register Temperature
pub const TEMP_OUT_H: u8 = 0x41;
/// Slave address of Mpu6050
pub const DEFAULT_SLAVE_ADDR: u8 = 0x68;
/// Internal register to check slave addr
pub const WHOAMI: u8 = 0x75;

/// Describes a bit block from bit number 'bit' to 'bit'+'length'
pub struct BitBlock {
    pub bit: u8,
    pub length: u8,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
/// Register 26: Configuration (DLPF, External signal)
pub struct CONFIG;

impl CONFIG {
    /// Base Address
    pub const ADDR: u8 = 0x1a;
    /// external Frame Synchronisation (FSYNC)
    pub const EXT_SYNC_SET: BitBlock = BitBlock { bit: 5, length: 3 };
    /// Digital Low Pass Filter (DLPF) config
    pub const DLPF_CFG: BitBlock = BitBlock { bit: 2, length: 3 };
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
    pub const FS_SEL: BitBlock = BitBlock { bit: 4, length: 2 };
    /// Accel Config ACCEL_HPF
    pub const ACCEL_HPF: BitBlock = BitBlock { bit: 2, length: 3 };
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
    pub const ACCEL_ON_DELAY: BitBlock = BitBlock { bit: 5, length: 2 };
    ///  Free Fall count
    pub const FF_COUNT: BitBlock = BitBlock { bit: 3, length: 2 };
    /// Motion Detection cound
    pub const MOT_COUNT: BitBlock = BitBlock { bit: 1, length: 2 };
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
    pub const LP_WAKE_CTRL: BitBlock = BitBlock { bit: 7, length: 2 };
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
    /// 1.25 Hz
    _1P25 = 0,
    /// 2.5 Hz
    _2P5,
    /// 5 Hz
    _5,
    /// 10 Hz
    _10,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// Accelerometer High Pass Filter Values
pub enum ACCEL_HPF {
    /// Cut off frequency: None
    _RESET = 0,
    /// Cut off frequency: 5 Hz
    _5 = 1,
    /// Cut off frequency: 2.5 Hz
    _2P5 = 2,
    /// Cut off frequency: 1.25 Hz
    _1P25 = 3,
    /// Cut off frequency: 0.63 Hz
    _0P63 = 4,
    /// When triggered, the filter holds the present sample. The filter output will be the
    /// difference between the input sample and the held sample
    _HOLD = 7,
}

impl From<u8> for ACCEL_HPF {
    fn from(range: u8) -> Self {
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
    /// Internal 8MHz oscillator
    OSCILL = 0,
    /// PLL with X axis gyroscope reference
    GXAXIS = 1,
    /// PLL with Y axis gyroscope reference
    GYAXIS = 2,
    /// PLL with Z axis gyroscope reference
    GZAXIS = 3,
    /// PLL with external 32.768kHz reference
    EXT_32p7 = 4,
    /// PLL with external 19.2MHz reference
    EXT_19P2 = 5,
    /// Reserved
    RESERV = 6,
    /// Stops the clock and keeps the timing generator in reset
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
            _ => CLKSEL::GXAXIS,
        }
    }
}

/// Defines accelerometer range/sensivity
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum AccelRange {
    /// 2G
    G2 = 0,
    /// 4G
    G4,
    /// 8G
    G8,
    /// 16G
    G16,
}

/// Defines gyro range/sensitivity
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum GyroRange {
    /// 250 degrees
    D250 = 0,
    /// 500 degrees
    D500,
    /// 1000 degrees
    D1000,
    /// 2000 degrees
    D2000,
}

impl From<u8> for GyroRange {
    fn from(range: u8) -> Self {
        match range {
            0 => GyroRange::D250,
            1 => GyroRange::D500,
            2 => GyroRange::D1000,
            3 => GyroRange::D2000,
            _ => GyroRange::D250,
        }
    }
}

impl From<u8> for AccelRange {
    fn from(range: u8) -> Self {
        match range {
            0 => AccelRange::G2,
            1 => AccelRange::G4,
            2 => AccelRange::G8,
            3 => AccelRange::G16,
            _ => AccelRange::G2,
        }
    }
}

impl AccelRange {
    // Converts accelerometer range to correction/scaling factor, see register sheet
    pub(crate) fn sensitivity(&self) -> f32 {
        match &self {
            AccelRange::G2 => ACCEL_SENS.0,
            AccelRange::G4 => ACCEL_SENS.1,
            AccelRange::G8 => ACCEL_SENS.2,
            AccelRange::G16 => ACCEL_SENS.3,
        }
    }
}

impl GyroRange {
    // Converts gyro range to correction/scaling factor, see register sheet
    pub(crate) fn sensitivity(&self) -> f32 {
        match &self {
            GyroRange::D250 => GYRO_SENS.0,
            GyroRange::D500 => GYRO_SENS.1,
            GyroRange::D1000 => GYRO_SENS.2,
            GyroRange::D2000 => GYRO_SENS.3,
        }
    }
}
