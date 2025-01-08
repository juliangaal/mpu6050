use mpu6050::{*, device::MOT_DETECT_STATUS};
use linux_embedded_hal::{I2cdev, Delay};
use embedded_hal::delay::DelayNs;
use i2cdev::linux::{LinuxI2CError};

fn main() -> Result<(), Mpu6050Error<LinuxI2CError>> {
    let i2c = I2cdev::new("/dev/i2c-1")
        .map_err(Mpu6050Error::I2c)?;

    let mut delay = Delay;
    let mut mpu = Mpu6050::new(i2c);
    
    mpu.init(&mut delay).unwrap();
    mpu.setup_motion_detection().unwrap();

    let mut count: u8 = 0;

    loop {
        if mpu.get_motion_detected().unwrap() {
            println!("YEAH BUDDY. Motion by axes: {:b}", mpu.read_byte(MOT_DETECT_STATUS::ADDR).unwrap());
            count += 1;
        }

        delay.delay_ms(10u32);

        if count > 5 {
            mpu.reset_device(&mut delay).unwrap();
            break;
        }
    }

    Ok(())
}
