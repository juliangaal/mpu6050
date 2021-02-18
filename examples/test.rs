use mpu6050::*;

use linux_embedded_hal::{I2cdev, Delay};
use i2cdev::linux::LinuxI2CError;
use mpu6050::device::ACCEL_HPF;

fn main() -> Result<(), Mpu6050Error<LinuxI2CError>> {
    let i2c = I2cdev::new("/dev/i2c-1")
        .map_err(Mpu6050Error::I2c)?;

    let mut delay = Delay;
    let mut mpu = Mpu6050::new(i2c);
    
    mpu.init(&mut delay)?;

    // Test gyro config
    assert_eq!(mpu.get_gyro_range()?, range::GyroRange::D250);
    mpu.set_gyro_range(range::GyroRange::D500)?;
    assert_eq!(mpu.get_gyro_range()?, range::GyroRange::D500);

    // Test accel config
    assert_eq!(mpu.get_accel_range()?, range::AccelRange::G2);
    mpu.set_accel_range(range::AccelRange::G4)?;
    assert_eq!(mpu.get_accel_range()?, range::AccelRange::G4);

    // accel_hpf
    assert_eq!(mpu.get_accel_hpf()?, ACCEL_HPF::_RESET);
    mpu.set_accel_hpf(ACCEL_HPF::_1P25);
    assert_eq!(mpu.get_accel_hpf()?, ACCEL_HPF::_1P25);

    println!("Test successful");
    Ok(())
}