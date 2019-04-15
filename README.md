# MPU 6050 Rust Driver <img align="right" width="300" height="250" src="https://www.invensense.com/wp-content/uploads/2015/01/rp-mpu-6500.png">

Platform agnostic driver for [MPU 6050 6-axis IMU](https://www.invensense.com/products/motion-tracking/6-axis/mpu-6500/) using [`embedded_hal`](https://github.com/rust-embedded/embedded-hal).

[Datasheet](https://www.invensense.com/wp-content/uploads/2015/02/MPU-6500-Datasheet2.pdf) | [Register Map Sheet](https://www.invensense.com/wp-content/uploads/2015/02/MPU-6000-Register-Map1.pdf)

[Docs](https://docs.rs/mpu6050/0.1.0/mpu6050/) | [Crate](https://crates.io/crates/mpu6050)

### Basic usage - [`linux_embedded_hal`](https://github.com/rust-embedded/linux-embedded-hal) example
```rust
use mpu6050::*;
use linux_embedded_hal::{I2cdev, Delay};
use i2cdev::linux::LinuxI2CError;

fn main() -> Result<(), Error<LinuxI2CError>> {
    let i2c = I2cdev::new("/dev/i2c-1") // or privide your owm on different platforms
        .map_err(Error::I2c)?;

    let delay = Delay;

    let mut mpu = Mpu6050::new(i2c, delay);
    mpu.init()?;

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
```
#### Compile linux example (Rapsberry Pi 3B)
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
- [x] device verification
- [ ] basic feature support as described [here](https://github.com/Tijndagamer/mpu6050/blob/master/mpu6050/mpu6050.py)
- [x] read gyro data
- [x] read acc data
- [x] software calibration
- [x] roll, pitch estimation accelerometer only
- [x] read temp data
- [ ] rename constants to better match datasheet
- [ ] complementary filter for roll, pitch estimate
- [ ] sample rate devider with register 25? or timer/clock control with PWR_MGMT_2
  - [ ] internal clock, register 108 `POWER_MGMT_2`, [will  cycle between  sleep mode  and  waking  up  to  take a single  sample of data from active sensors at a rate determined by LP_WAKE_CTRL](https://www.invensense.com/wp-content/uploads/2015/02/MPU-6000-Register-Map1.pdf) (page 41-43)
- [ ] plotting [csv data](https://plot.ly/python/plot-data-from-csv/)for testing?

