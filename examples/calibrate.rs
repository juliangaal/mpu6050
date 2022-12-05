// Finds offset values to calibrate the sensor.
// Before running this example, you should let your mpu6050 warm up for around 10 minutes until it
// reaches a stable temperature (because temperature affects sensor readings).

use mpu6050::*;
use device::{AccelRange, GyroRange};
use linux_embedded_hal::{I2cdev, i2cdev::linux::LinuxI2CError, Delay};
use embedded_hal::blocking::delay::DelayMs;
use nalgebra::Vector3;

const ITERATIONS: i32 = 1000;
const ACC_ZERO: Vector3<f32> = Vector3::new(0.0, 0.0, 1.0);
const GYRO_ZERO: Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);
//This value determines how precise the offsets are. By making this value smaller, you can increase
// the precision of the offsets, but you should also increase the number of iterations accordingly.
const SPEED: f32 = 0.1;
//The number of milliseconds between each reading.
const DELAY: u16 = 10;

const TEST_ITERATIONS: i32 = 1000;


fn main() -> Result<(), Mpu6050Error<LinuxI2CError>> {
	let i2c = I2cdev::new("/dev/i2c-1").map_err(Mpu6050Error::I2c)?;
	let mut calib = mpu6050::CalibrationData::default();
	let mut calib_test = mpu6050::CalibrationData::default();

	let mut delay = Delay;
	let mut mpu = Mpu6050::new_with_sens(i2c,
										 AccelRange::G2, GyroRange::D250
	);

	mpu.init(&mut delay)?;
	let mut calib = CalibrationData::default();

	//Calculate offsets
	println!("Calculating offsets");
	for i in 0..ITERATIONS {
		let acc = mpu.get_acc()?;
		let gyro = mpu.get_gyro()?;

		let delta_acc = ACC_ZERO - acc;
		calib.acc_offsets += delta_acc * SPEED;
		let delta_gyro = GYRO_ZERO - gyro;
		calib.gyro_offsets += delta_gyro * SPEED;

		mpu.load_calibration_data(calib)?;

		if i % (ITERATIONS / 10) == 0 {
			println!("{} Iterations \ncurrent Acc_offsets: {}, current Gyro_offsets: {}", i,
					 calib.acc_offsets, calib.gyro_offsets);
		}

		delay.delay_ms(DELAY);
	}
	println!("---FINISHED---");
	println!("Offsets: acc: {}, gyro: {}", calib.acc_offsets, calib.gyro_offsets);

	//Testing offsets
	println!("---TESTING---");
	let inverse_i = 1.0 / TEST_ITERATIONS as f32;
	let mut error = 0.0;

	for i in 0..TEST_ITERATIONS {
		let acc = mpu.get_acc()?;
		let gyro = mpu.get_gyro()?;

		let acc_diff = ACC_ZERO - acc;
		error += (acc_diff.component_mul(&acc_diff)).sum() * inverse_i;

		let gyro_diff = GYRO_ZERO - gyro;
		error += (gyro_diff.component_mul(&gyro_diff)).sum() * inverse_i;

		if i % (TEST_ITERATIONS / 3) == 0 {
			println!("current Acc_readings: {}, current Gyro_readings: {}", acc, gyro);
		}

		delay.delay_ms(DELAY);
	}

	println!("---FINISHED---");
	println!("average error: {}", error);
	Ok(())
}
