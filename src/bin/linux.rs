use mpu6050::*;
use linux_embedded_hal::{I2cdev, Delay};
use i2cdev::linux::LinuxI2CError;

fn main() -> Result<(), Error<LinuxI2CError>> {
    let i2c = I2cdev::new("/dev/i2c-1")
        .map_err(Error::I2c)?;

    let delay = Delay;

    let mut mpu = Mpu6050::new(i2c, delay);
    mpu.init()?;
    //mpu.soft_calibrate(200)?;

    loop {
        match mpu.get_acc_angles() {
            Ok(r) => {
                println!("{:?}", r);
            },
            Err(_) => {} ,
        }
    }
}
