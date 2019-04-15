use mpu6050::*;
use linux_embedded_hal::{I2cdev, Delay};
use i2cdev::linux::LinuxI2CError;

fn main() -> Result<(), Error<LinuxI2CError>> {
    let i2c = I2cdev::new("/dev/i2c-1")
        .map_err(Error::I2c)?;

    let delay = Delay;

    let mut mpu = Mpu6050::new(i2c, delay);
    mpu.init()?;
    //mpu.soft_calib(200)?;

    loop {
        // get roll and pitch estimate
        match mpu.get_acc_angles() {
            Ok(r) => {
                println!("r/p: {:?}", r);
            },
            Err(_) => {} ,
        }

        // get temp
        match mpu.get_temp() {
            Ok(r) => {
                println!("temp: {}c", r);
            },
            Err(_) => {} ,
        }

        // get gyro data, scaled with sensitivity 
        match mpu.get_gyro() {
            Ok(r) => {
                println!("gyro: {:?}", r);
            },
            Err(_) => {} ,
        }
        
        // get accelerometer data, scaled with sensitivity
        match mpu.get_acc() {
            Ok(r) => {
                println!("acc: {:?}", r);
            },
            Err(_) => {} ,
        }
    }
}
