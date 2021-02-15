//! All constants used in the driver, mostly register addresses

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
