use mpu6050::*;

use linux_embedded_hal::{I2cdev, Delay};
use i2cdev::linux::LinuxI2CError;

fn main() -> Result<(), Mpu6050Error<LinuxI2CError>> {
    let i2c = I2cdev::new("/dev/i2c-1")
        .map_err(Mpu6050Error::I2c)?;

    let mut delay = Delay;
    let mut mpu = Mpu6050::new(i2c);
    
    mpu.init(&mut delay)?;

    println!("{:#?}", mpu.get_gyro_range());
    mpu.set_gyro_range(range::GyroRange::D500)?;
    use std::{thread, time};
    let ten_millis = time::Duration::from_millis(1000);
    thread::sleep(ten_millis);
    println!("{:#?}", mpu.get_gyro_range());

    loop {
        // mpu.
    }
}

// MPU6050_RA_MOT_DETECT_CTRL, 0x69, = 3
// MPU6050_RA_INT_ENABLE,  0x38, = 1
// MPU6050_RA_ACCEL_CONFIG, 0x1C, = 1
// MPU6050_RA_MOT_THR, 0x1F, = 2
// MPU6050_RA_ZRMOT_DUR, 0x22, = 1