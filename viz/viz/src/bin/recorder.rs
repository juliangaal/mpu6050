use mpu6050::*;
use linux_embedded_hal::{I2cdev, Delay};
use i2cdev::linux::LinuxI2CError;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::error::Error;

fn new_file(name: &str) -> File {
    let path = Path::new(name);
    let display = path.display();
    let _file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           why.description()),
        Ok(file) => return file,
    };
}

fn write_x_to(file: &mut File, content: String) {
    match file.write_all(content.as_bytes()) {
        Err(why) => {
            println!("couldn't write to file: {}", why.description());
        },
        Ok(_) => {},
    }
}

fn main() -> Result<(), Mpu6050Error<LinuxI2CError>> {
    let i2c = I2cdev::new("/dev/i2c-1")
        .map_err(Mpu6050Error::I2c)?;

    let delay = Delay;

    let mut mpu = Mpu6050::new(i2c, delay);
    mpu.init()?;
    mpu.soft_calib(200)?;
    mpu.calc_variance(200)?;

    println!("Calculated variance: {:?}", mpu.get_variance().unwrap());

    let mut acc_file = new_file("acc_data.txt");
    let mut gyro_file = new_file("gyro_data.txt");
    let mut temp_file = new_file("temp_data.txt");
    let mut angles_file = new_file("angles_data.txt");

    loop {
        // get roll and pitch estimate
        let acc = mpu.get_acc_angles()?;
        write_x_to(&mut angles_file, format!("{},{}\n", acc.0, acc.1));

        // get temp
        let temp = mpu.get_temp()?;
        write_x_to(&mut temp_file, format!("{}\n", temp));

        // get gyro data, scaled with sensitivity 
        let gyro = mpu.get_gyro()?;
        write_x_to(&mut gyro_file, format!("{},{},{}\n", gyro.0, gyro.1, gyro.2));
       
        // get accelerometer data, scaled with sensitivity
        let acc = mpu.get_acc()?;
        write_x_to(&mut acc_file, format!("{},{},{}\n", acc.0, acc.1, acc.2));
    }
}
