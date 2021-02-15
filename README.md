# `mpu6050` ![crates.io](https://img.shields.io/crates/v/mpu6050.svg) ![CircleCI](https://img.shields.io/circleci/build/github/juliangaal/mpu6050.svg)
> no_std driver for the MPU6050 6-axis IMU

[Datasheet](https://www.invensense.com/wp-content/uploads/2015/02/MPU-6500-Datasheet2.pdf) | [Register Map Sheet](https://www.invensense.com/wp-content/uploads/2015/02/MPU-6000-Register-Map1.pdf)

Visualization options for testing and development in [viz branch](https://github.com/juliangaal/mpu6050/tree/viz/viz)

### Basic usage 
[`linux_embedded_hal`](https://github.com/rust-embedded/linux-embedded-hal) example
```rust
use mpu6050::*;
use linux_embedded_hal::{I2cdev, Delay};
use i2cdev::linux::LinuxI2CError;

fn main() -> Result<(), Mpu6050Error<LinuxI2CError>> {
  let i2c = I2cdev::new("/dev/i2c-1")
          .map_err(Mpu6050Error::I2c)?;

  let mut delay = Delay;
  let mut mpu = Mpu6050::new(i2c);

  mpu.init(&mut delay)?;

  loop {
    // get roll and pitch estimate
    let acc = mpu.get_acc_angles()?;
    println!("r/p: {:?}", acc);

    // get temp
    let temp = mpu.get_temp()?;
    println!("temp: {:?}c", temp);

    // get gyro data, scaled with sensitivity 
    let gyro = mpu.get_gyro()?;
    println!("gyro: {:?}", gyro);

    // get accelerometer data, scaled with sensitivity
    let acc = mpu.get_acc()?;
    println!("acc: {:?}", acc);
  }
}
```

### Linux example (Raspberry Pi 3B)

#### Cross compile
Requirements: 
```bash
$ apt-get install -qq gcc-arm-linux-gnueabihf libc6-armhf-cross libc6-dev-armhf-cross
```

Rustup:
```bash
$ rustup target add armv7-unknown-linux-gnueabihf
```
To configure the linker use `.cargo/config` file

cross-compile with 
```bash
$ cargo build --examples --target=armv7-unknown-linux-gnueabihf
```
Copy binary to rpi, once steps below are successfully completed.

#### On RPi (RPi OS lite, 15.02.21)

##### Install i2c dependencies
```bash
$ sudo apt-get install i2c-tools libi2c-dev python-smbus -y
```

##### Enable and load kernel modules at boot
1. edit `/etc/modules` and add
    ```
    i2c-dev
    i2c-bcm2708
    ```
2. edit `/boot/config.txt` and add/uncomment
    ```
    dtparam=i2c_arm=on
    dtparam=i2c1=on
    ```
3. reboot
4. check if working with `sudo i2cdetect -y 1` or `sudo i2cdetect -y 0`. You should bet a signal similar to this
    ```
      0 1 2 3 4 5 6 7 8 9 a b c d e f
    00: -- -- -- -- -- -- -- -- -- -- -- -- --
    10: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
    20: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
    30: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
    40: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
    50: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
    60: -- -- -- -- -- -- -- -- -- -- -- 68 -- -- -- --
    70: -- -- -- -- -- -- -- --
    ```
5. Wire up 

    | [RPi GPIO](https://www.raspberrypi.org/documentation/usage/gpio/) | MPU6050 |
    |:---|---:|
    | 5V Power | VCC |
    | Ground | GND |
    | GPIO 2 | SCL |
    | GPIO 3 | SDA |

## TODO
- [x] init with default settings
- [ ] init with custom settings
  - [x] custom sensitivity
  - [ ] custom device initialization
- [x] device verification
- [x] basic feature support as described [here](https://github.com/Tijndagamer/mpu6050/blob/master/mpu6050/mpu6050.py)
- [x] rename constants to better match datasheet
- [ ] complementary filter for roll, pitch estimate, possible on device?
- [ ] low pass filter registers? How to use?
- [ ] sample rate devider with register 25? or timer/clock control with PWR_MGMT_2
  - [ ] internal clock, register 108 `POWER_MGMT_2`, [will  cycle between  sleep mode  and  waking  up  to  take a single  sample of data from active sensors at a rate determined by LP_WAKE_CTRL](https://www.invensense.com/wp-content/uploads/2015/02/MPU-6000-Register-Map1.pdf) (page 41-43)
- [x] plotting [csv data](https://plot.ly/python/plot-data-from-csv/)for testing? -> See viz branch

