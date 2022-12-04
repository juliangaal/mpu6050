// Finds offset values to calibrate the sensor.
// Before running this example, you should let your mpu6050 warm up for around 10 minutes until it reaches a stable temperature
// (because temperature affects sensor readings).

use mpu6050::*;
use device::{AccelRange, GyroRange};
use linux_embedded_hal::{I2cdev, i2cdev::linux::LinuxI2CError, Delay};
use embedded_hal::blocking::delay::DelayMs;

//Change this line to the error type of your I2c backend.
type I2cErr = LinuxI2CError;

//Change this line to the name of the main struct of your I2c backend.
type I2c = I2cdev;

const ITERATIONS: i32 = 1000;
const ACC_ZERO: [f32; 3] = [0.0, 0.0, 1.0];
const GYRO_ZERO: [f32; 3] = [0.0, 0.0, 0.0];
const SPEED: f32 = 0.1; //This value determines how precise the offsets are. By making this value smaller, you can increase the precision of the offsets, but you should also increase the number of iterations accordingly.
const DELAY: u16 = 10; //The number of milliseconds between each reading.

const TEST_ITERATIONS: i32 = 1000;

fn main() -> Result<(), Mpu6050Error<I2cErr>> {
	let i2c = I2c::

	new("/dev/i2c-1") //Change this line to whatever is required to create your I2c backend.

	.map_err(Mpu6050Error::I2c)?;
	let mut delay = Delay;
	let mut mpu = Mpu6050::new_with_sens(i2c, 

		AccelRange::G2, GyroRange::D250 //Change these sensitivities to the ones you use.

	);

	mpu.init(&mut delay)?;
	println!("---CALCULATING OFFSETS---");
	//Calculate offsets
	for i in 0..ITERATIONS {
		let acc = mpu.get_acc()?;
		let gyro = mpu.get_gyro()?;
		
		let delta = ACC_ZERO[0] - acc.x;
		mpu.acc_offsets.x += delta * SPEED;
		let delta = ACC_ZERO[1] - acc.y;
		mpu.acc_offsets.y += delta * SPEED;
		let delta = ACC_ZERO[2] - acc.z;
		mpu.acc_offsets.z += delta * SPEED;

		let delta = GYRO_ZERO[0] - gyro.x;
		mpu.gyro_offsets.x += delta * SPEED;
		let delta = GYRO_ZERO[1] - gyro.y;
		mpu.gyro_offsets.y += delta * SPEED;
		let delta = GYRO_ZERO[2] - gyro.z;
		mpu.gyro_offsets.z += delta * SPEED;

		if i % (ITERATIONS / 10) == 0 {println!("{} Iterations \ncurrent Acc_offsets: {}, current Gyro_offsets: {}", i, mpu.acc_offsets, mpu.gyro_offsets);}

		delay.delay_ms(DELAY);


	}
	println!("---FINISHED---");
	println!("Offsets: acc: {}, gyro: {}", mpu.acc_offsets, mpu.gyro_offsets);
	//Testing offsets
	println!("---TESTING---");
	let inverse_i = 1.0 / TEST_ITERATIONS as f32;
	let mut error = 0.0;
	for i in 0..TEST_ITERATIONS {
		let acc = mpu.get_acc()?;
		let gyro = mpu.get_gyro()?;
		
		error += {let delta = ACC_ZERO[0] - acc.x; delta * delta} * inverse_i;
		error += {let delta = ACC_ZERO[1] - acc.y; delta * delta} * inverse_i;
		error += {let delta = ACC_ZERO[2] - acc.z; delta * delta} * inverse_i;

		error += {let delta = GYRO_ZERO[0] - gyro.x; delta * delta} * inverse_i;
		error += {let delta = GYRO_ZERO[1] - gyro.y; delta * delta} * inverse_i;
		error += {let delta = GYRO_ZERO[2] - gyro.z; delta * delta} * inverse_i;

		if i % (TEST_ITERATIONS / 3) == 0 {println!("current Acc_readings: {}, current Gyro_readings: {}", acc, gyro);}

		delay.delay_ms(DELAY);
	}
	
	println!("---FINISHED---");
	println!("average error: {}", error);
	println!("\nYou can copy and paste the following into your code:\n");
	println!("//Accelerometer offsets of Mpu6050");
	println!("mpu.acc_offsets.x = {};", mpu.acc_offsets.x);
	println!("mpu.acc_offsets.y = {};", mpu.acc_offsets.y);
	println!("mpu.acc_offsets.z = {};", mpu.acc_offsets.z);
	println!("//Gyroscope offsets of Mpu6050");
	println!("mpu.gyro_offsets.x = {};", mpu.gyro_offsets.x);
	println!("mpu.gyro_offsets.y = {};", mpu.gyro_offsets.y);
	println!("mpu.gyro_offsets.z = {};", mpu.gyro_offsets.z);
	Ok(())
}
