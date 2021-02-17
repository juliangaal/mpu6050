// Mostly taken from https://github.com/jrowberg/i2cdevlib/blob/master/Arduino/I2Cdev/I2Cdev.cpp
// and tested

pub fn get_bit_n(byte: &[u8; 1], n: u8) -> u8 {
    (byte[0] >> n) & 1
}

pub fn set_bit_n(byte: &mut [u8; 1], n: u8, enable: bool) {
    if enable {
        byte[0] |= 1_u8 << n;
    } else {
        byte[0] &= !(1_u8 << n);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_bit_n_test() {
        let byte = 4_u8.to_be_bytes();
        assert_eq!(get_bit_n(&byte, 2), 1);
        assert_eq!(get_bit_n(&byte, 1), 0);
        assert_eq!(get_bit_n(&byte, 0), 0);
    }

    #[test]
    fn set_bit_n_test() {
        let mut byte = 4_u8.to_be_bytes();

        // enable bit 1
        set_bit_n(&mut byte, 1, true);
        assert_eq!(byte[0], 6);

        // disable bit 1
        set_bit_n(&mut byte, 1, false);
        assert_eq!(byte[0], 4);

        // enable bit 3
        set_bit_n(&mut byte, 3, true);
        assert_eq!(byte[0], 12);
    }
}
