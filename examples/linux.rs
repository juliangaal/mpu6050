use mpu6050::*;
use linux_embedded_hal::{I2cdev, Delay};
use i2cdev::linux::LinuxI2CError;

fn main() -> Result<(), Mpu6050Error<LinuxI2CError>> {
    let i2c = I2cdev::new("/dev/i2c-1")
        .map_err(Mpu6050Error::I2c)?;

    let delay = Delay;

    let mut mpu = Mpu6050::new(i2c, delay);
    mpu.init()?;
    mpu.soft_calib(Steps(100))?;
    mpu.calc_variance(Steps(50))?;

    println!("Calibrated with bias: {:?}", mpu.get_bias().unwrap());
    println!("Calculated variance: {:?}", mpu.get_variance().unwrap());

    loop {
        // get roll and pitch estimate
        let acc = mpu.get_acc_angles()?;
        println!("r/p: {?}", acc);

        // get roll and pitch estimate - averaged accross n readings (steps)
        let acc = mpu.get_acc_angles_avg(Steps(5))?;
        println!("r/p avg: {?}", acc);

        // get temp
        let temp = mpu.get_temp()?;
        println!("temp: {}c", temp);

        // get temp - averages across n readings (steps)
        let temp = mpu.get_temp_avg(Steps(5))?;
        println!("temp avg: {}c", temp);

        // get gyro data, scaled with sensitivity
        let gyro = mpu.get_gyro()?;
        println!("gyro: {?}", gyro);

        // get gyro data, scaled with sensitivity - averaged across n readings (steps)
        let gyro = mpu.get_gyro_avg(Steps(5))?;
        println!("gyro avg: {?}", gyro);

        // get accelerometer data, scaled with sensitivity
        let acc = mpu.get_acc()?;
        println!("acc: {?}", acc);

        // get accelerometer data, scaled with sensitivity - averages across n readings (steps)
        let acc = mpu.get_acc_avg(Steps(5))?;
        println!("acc avg: {?}", acc);
    }
}
