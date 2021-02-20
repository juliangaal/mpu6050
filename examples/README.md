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
