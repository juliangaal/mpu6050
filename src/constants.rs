pub const SLAVE_ADDR: u8 = 0x68;
pub const WHOAMI: u8 = 0x75;

/// High Bytle Register Gyro x orientation
pub const GYRO_REGX_H: u8 = 0x43;
/// High Bytle Register Gyro y orientation
pub const GYRO_REGY_H: u8 = 0x45;
/// High Bytle Register Gyro z orientation
pub const GYRO_REGZ_H: u8 = 0x47;
 
/// High Byte Register Calc roll
pub const ACC_REGX_H: u8 = 0x3b;
/// High Byte Register Calc pitch
pub const ACC_REGY_H: u8 = 0x3d;
/// High Byte Register Calc yaw
pub const ACC_REGZ_H: u8 = 0x3f;

/// High Byte Register Temperature
pub const TEMP_OUT_H: u8 = 0x41;

/// Register to control chip waking from sleep, enabling sensors, default: sleep
pub const POWER_MGMT_1: u8 = 0x6b; 
/// Internal clock
pub const POWER_MGMT_2: u8 = 0x6c; 

/// Gyro Sensitivity
pub const FS_SEL: (f32, f32, f32, f32) = (131., 65.5, 32.8, 16.4);
/// Calcelerometer Sensitivity
pub const AFS_SEL: (f32, f32, f32, f32) = (16384., 8192., 4096., 2048.);

pub const PI: f32 = 3.14159;

pub const GRAVITIY_MS2: f32 = 9.80665;

pub const ACCEL_RANGE_2G: u8 = 0x00;
pub const ACCEL_RANGE_4G: u8 = 0x08;
pub const ACCEL_RANGE_8G: u8 = 0x10;
pub const ACCEL_RANGE_16G: u8 = 0x18;

pub const GYRO_RANGE_250DEG: u8 = 0x00;
pub const GYRO_RANGE_500DEG: u8 = 0x08;
pub const GYRO_RANGE_1000DEG: u8 = 0x10;
pub const GYRO_RANGE_2000DEG: u8 = 0x18;

pub const ACCEL_CONFIG: u8  = 0x1c;
pub const GYRO_CONFIG: u8 = 0x1b;
