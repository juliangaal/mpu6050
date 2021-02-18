use crate::device::*;

/// Defines accelerometer range/sensivity
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum AccelRange {
    G2 = 0,
    G4,
    G8,
    G16,
}

/// Defines gyro range/sensitivity
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum GyroRange {
    D250 = 0,
    D500,
    D1000,
    D2000,
}

impl From<u8> for GyroRange {
    fn from(range: u8) -> Self
    {
        match range {
            0 => GyroRange::D250,
            1 => GyroRange::D500,
            2 => GyroRange::D1000,
            3 => GyroRange::D2000,
            _ => GyroRange::D250
        }
    }
}

impl From<u8> for AccelRange {
    fn from(range: u8) -> Self
    {
        match range {
            0 => AccelRange::G2,
            1 => AccelRange::G4,
            2 => AccelRange::G8,
            3 => AccelRange::G16,
            _ => AccelRange::G2
        }
    }
}

impl AccelRange {
    pub(crate) fn sensitivity(&self) -> f32 {
        match &self {
            AccelRange::G2 => ACCEL_SENS.0,
            AccelRange::G4 => ACCEL_SENS.1,
            AccelRange::G8 => ACCEL_SENS.2,
            AccelRange::G16 => ACCEL_SENS.3,
        }
    }
}

// Converts gyro range to correction/scaling factor, see table p. 31 or register sheet
impl GyroRange {
    pub(crate) fn sensitivity(&self) -> f32 {
        match &self {
            GyroRange::D250 => GYRO_SENS.0,
            GyroRange::D500 => GYRO_SENS.1,
            GyroRange::D1000 => GYRO_SENS.2,
            GyroRange::D2000 => GYRO_SENS.3,
        }
    }
}