use crate::range::*;

/// Gyro Sensitivity
pub const GYRO_SENS: (f32, f32, f32, f32) = (131., 65.5, 32.8, 16.4);

/// Accelerometer Sensitivity
pub const ACCEL_SENS: (f32, f32, f32, f32) = (16384., 8192., 4096., 2048.);

/// Temperature Offset
pub const TEMP_OFFSET: f32 = 36.53;

/// Temperature Sensitivity
pub const TEMP_SENSITIVITY: f32 = 340.;

// Helper struct to convert Sensor measurement range to appropriate values defined in datasheet
pub(crate) struct Sensitivity(pub(crate) f32);

// Converts accelerometer range to correction/scaling factor, see table p. 29 or register sheet
impl From<AccelRange> for Sensitivity {
    fn from(range: AccelRange) -> Sensitivity {
        match range {
            AccelRange::G2 => return Sensitivity(ACCEL_SENS.0),
            AccelRange::G4 => return Sensitivity(ACCEL_SENS.1),
            AccelRange::G8 => return Sensitivity(ACCEL_SENS.2),
            AccelRange::G16 => return Sensitivity(ACCEL_SENS.3),
        }
    }
}

// Converts gyro range to correction/scaling factor, see table p. 31 or register sheet
impl From<GyroRange> for Sensitivity {
    fn from(range: GyroRange) -> Self {
        match range {
            GyroRange::D250 => return Sensitivity(GYRO_SENS.0),
            GyroRange::D500 => return Sensitivity(GYRO_SENS.1),
            GyroRange::D1000 => return Sensitivity(GYRO_SENS.2),
            GyroRange::D2000 => return Sensitivity(GYRO_SENS.3),
        }
    }
}