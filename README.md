# mpu6050

Platform agnostic driver for MPU6050 sensor.

### Linux example
Requirements: `apt-get install -qq gcc-arm-linux-gnueabihf libc6-armhf-cross libc6-dev-armhf-cross`
cross-compile with `cargo build --bin main --target=arm-unknown-linux-gnueabihf`

## TODO
- [x] init with default settings
- [ ] init with custom settings
- [x] device verification
- [x] basic feature support as described [here](https://github.com/Tijndagamer/mpu6050/blob/master/mpu6050/mpu6050.py)
- [ ] read gyro data
- [ ] read acc data
- [ ] read temp data
- [ ] timer/clock control with PWR_MGMT_2
  - [ ] internal clock, register 108 `POWER_MGMT_2`, [will  cycle between  sleep mode  and  waking  up  to  take a single  sample of data from active sensors at a rate determined by LP_WAKE_CTRL](https://www.invensense.com/wp-content/uploads/2015/02/MPU-6000-Register-Map1.pdf) (page 41-43)
- [ ] plotting [csv data](https://plot.ly/python/plot-data-from-csv/)?
- [ ] complementary filter
- [ ] time step

