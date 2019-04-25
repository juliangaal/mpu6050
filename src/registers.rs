//! All constants used in the driver, mostly register addresses

/// Slave address of Mpu6050
pub const SLAVE_ADDR: u8 = 0x68;
/// Internal register to check slave addr
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

/// Accelerometer config register
pub const ACCEL_CONFIG: u8  = 0x1c;

/// gyro config register
pub const GYRO_CONFIG: u8 = 0x1b;

/// pi
pub const PI: f32 = 3.14159265358979323846;
