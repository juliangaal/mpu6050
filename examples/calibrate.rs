// Finds offset values to calibrate the sensor.
// Before running this example, you should let your mpu6050 warm up for around 10 minutes until it reaches a stable temperature
// (because temperature affects sensor readings).

use mpu6050::*;
use linux_embedded_hal::{I2cdev, Delay};
use i2cdev::linux::LinuxI2CError;
use embedded_hal::blocking::delay::DelayMs;

//Change this line to the error type of your I2c backend.
type I2cErr = LinuxI2CError;

//Change this line to the name of the main struct of your I2c backend.
type I2c = I2cdev;

const iterations: i32 = 1000;
const acc_zero: [f32; 3] = [0.0, 0.0, 1.0];
const gyro_zero: [f32; 3] = [0.0, 0.0, 0.0];
const speed = 0.1; //This value determines how precise the offsets are. By making this value smaller, you can increase the precision of the offsets, but you should also increase the number of iterations accordingly.

fn main() -> Result<(), Mpu6050Error<I2cErr>> {
	let i2c = I2c::

	new("/dev/i2c-1") //Change this line to whatever is required to create your I2c backend.

	.map_err(Mpu6050Error::I2c)?;
	let mut delay = Delay;
	let mut mpu = Mpu6050::new_with_sens(i2c, 

		AccelRange::G2, GyroRange::D250 //Change these sensitivities to the ones you use.

	);

	mpu.init(&mut delay)?;
	for i in 0..iterations {
		let acc = mpu.get_acc()?;
		let gyro = mpu.get_gyro()?;
		
		let delta = acc_zero.0 - acc.x;
		mpu.acc_offsets.x += delta * speed;
		let delta = acc_zero.1 - acc.y;
		mpu.acc_offsets.y += delta * speed;
		let delta = acc_zero.2 - acc.z;
		mpu.acc_offsets.z += delta * speed;

		let delta = gyro_zero.0 - gyro.x;
		mpu.gyro_offsets.x += delta * speed;
		let delta = gyro_zero.1 - gyro.y;
		mpu.gyro_offsets.y += delta * speed;
		let delta = gyro_zero.2 - gyro.z;
		mpu.gyro_offsets.z += delta * speed;

		if i % (iterations / 10) == 0 {println!("{} Iterations \nAcc_offsets: {}, Gyro_offsets: {}", i, mpu.acc_offsets, mpu.gyro_offsets);}

		delay.delay_ms(2);


	}
	println!("Offsets: acc: {}, gyro: {}", mpu.acc_offsets, mpu.gyro_offsets);
	Ok(())
}
