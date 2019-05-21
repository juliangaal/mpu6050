# `mpu6050` ![crates.io](https://img.shields.io/crates/v/mpu6050.svg)
> no_std driver for the MPU6050 6-axis IMU

[Datasheet](https://www.invensense.com/wp-content/uploads/2015/02/MPU-6500-Datasheet2.pdf) | [Register Map Sheet](https://www.invensense.com/wp-content/uploads/2015/02/MPU-6000-Register-Map1.pdf)

Visualization options for testing and development in [viz branch](https://github.com/juliangaal/mpu6050/tree/viz/viz)

### Basic usage 
[`linux_embedded_hal`](https://github.com/rust-embedded/linux-embedded-hal) example
```rust
use mpu6050::*;
use linux_embedded_hal::{I2cdev, Delay};
use i2cdev::linux::LinuxI2CError;

fn main() -> Result<(), Error<LinuxI2CError>> {
    let i2c = I2cdev::new("/dev/i2c-1")
        .map_err(Error::I2c)?;

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
        println!("r/p: {:?}", acc);

        // get roll and pitch estimate - averaged accross n readings (steps)
        let acc = mpu.get_acc_angles_avg(Steps(5))?;
        println!("r/p avg: {:?}", acc);

        // get temp
        let temp = mpu.get_temp()?;
        println!("temp: {}c", temp);

        // get temp - averages across n readings (steps)
        let temp = mpu.get_temp_avg(Steps(5))?;
        println!("temp avg: {}c", temp);

        // get gyro data, scaled with sensitivity 
        let gyro = mpu.get_gyro()?;
        println!("gyro: {:?}", gyro);
        
        // get gyro data, scaled with sensitivity - averaged across n readings (steps) 
        let gyro = mpu.get_gyro_avg(Steps(5))?;
        println!("gyro avg: {:?}", gyro);
        
        // get accelerometer data, scaled with sensitivity
        let acc = mpu.get_acc()?;
        println!("acc: {:?}", acc);
        
        // get accelerometer data, scaled with sensitivity - averages across n readings (steps)
        let acc = mpu.get_acc_avg(Steps(5))?;
        println!("acc avg: {:?}", acc);

}
```

#### Compile linux example (Raspberry Pi 3B)
files [here](https://github.com/juliangaal/mpu6050/blob/master/example/)

Requirements: 
```bash
$ apt-get install -qq gcc-armv7-linux-gnueabihf libc6-armhf-cross libc6-dev-armhf-cross
```
and all dependencies in `example/Cargo.toml`

Rustup:
```bash
$ rustup target add armv7-unknown-linux-gnueabihf
```
To configure the linker use `example/.cargo/config`

cross-compile with 
```bash
$ cargo build --target=armv7-unknown-linux-gnueabihf
```

## TODO
- [x] init with default settings
- [ ] init with custom settings
  - [x] custom sensitivity
  - [ ] custom device initialization
- [x] device verification
- [x] basic feature support as described [here](https://github.com/Tijndagamer/mpu6050/blob/master/mpu6050/mpu6050.py)
- [x] read gyro data
- [x] read acc data
- [x] software calibration
- [x] software measurement variance estimation
- [x] roll, pitch estimation accelerometer only
- [x] read temp data
- [ ] rename constants to better match datasheet
- [ ] complementary filter for roll, pitch estimate, possible on device? 
- [ ] sample rate devider with register 25? or timer/clock control with PWR_MGMT_2
  - [ ] internal clock, register 108 `POWER_MGMT_2`, [will  cycle between  sleep mode  and  waking  up  to  take a single  sample of data from active sensors at a rate determined by LP_WAKE_CTRL](https://www.invensense.com/wp-content/uploads/2015/02/MPU-6000-Register-Map1.pdf) (page 41-43)
- [x] plotting [csv data](https://plot.ly/python/plot-data-from-csv/)for testing? -> See viz branch

