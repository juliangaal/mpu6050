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

    // Test power management
    println!("Test power management");

    // Test gyro config
    println!("Test gyro config");
    assert_eq!(mpu.get_gyro_range()?, range::GyroRange::D250);
    mpu.set_gyro_range(range::GyroRange::D500)?;
    assert_eq!(mpu.get_gyro_range()?, range::GyroRange::D500);

    // Test accel config
    println!("Test accel config");
    assert_eq!(mpu.get_accel_range()?, range::AccelRange::G2);
    mpu.set_accel_range(range::AccelRange::G4)?;
    assert_eq!(mpu.get_accel_range()?, range::AccelRange::G4);

    // accel_hpf
    println!("Test accel hpf");
    assert_eq!(mpu.get_accel_hpf()?, ACCEL_HPF::_RESET);
    mpu.set_accel_hpf(ACCEL_HPF::_1P25)?;
    assert_eq!(mpu.get_accel_hpf()?, ACCEL_HPF::_1P25);

    // test sleep
    println!("Test sleep");
    assert_eq!(mpu.get_sleep_enabled()?, false);
    mpu.set_sleep_enabled(true)?;
    assert_eq!(mpu.get_sleep_enabled()?, true);
    mpu.set_sleep_enabled(false)?;
    assert_eq!(mpu.get_sleep_enabled()?, false);
    // mpu.set_sleep_enabled(true)?;

    // test temp enable/disable
    println!("Test temp enable/disable");
    mpu.set_temp_enabled(false)?;
    assert_eq!(mpu.get_temp_enabled()?, false);
    mpu.set_temp_enabled(true)?;
    assert_eq!(mpu.get_temp_enabled()?, true);

    // reset
    println!("Test reset");
    mpu.reset_device(&mut delay)?;
    assert_eq!(mpu.get_accel_hpf()?, ACCEL_HPF::_RESET);
    assert_eq!(mpu.get_accel_range()?, range::AccelRange::G2);
    assert_eq!(mpu.get_gyro_range()?, range::GyroRange::D250);
    assert_eq!(mpu.get_sleep_enabled()?, true);
    assert_eq!(mpu.get_temp_enabled()?, false);

    println!("Test successful");
    Ok(())
}