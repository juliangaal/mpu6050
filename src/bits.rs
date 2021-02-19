//! Bit operations on registers
//! Mostly taken from https://github.com/jrowberg/i2cdevlib/blob/master/Arduino/I2Cdev/I2Cdev.cpp
//! updated and tested

/// get bit n of byte
pub fn get_bit(byte: u8, n: u8) -> u8 {
    (byte >> n) & 1
}

/// get bits start - start+length from byte
pub fn get_bits(mut byte: u8, bit_start: u8, length: u8) -> u8 {
    // 01101001 read byte
    // 76543210 bit numbers
    //    xxx   args: bit_start=4, length=3
    //    010   masked
    //   -> 010 shifted

    // without mask_shift, strange behavior occurs, wenn bit_start < length.
    // e.g. bit_start=2, length = 2
    // in SOME cases, you get an 'attempt to subtract with overflow' exception, when
    // bitstart - length + 1 = 0
    // therefore just "cut off" at 0 shift
    let mask_shift: u8 = if bit_start < length { 0 } else { bit_start - length + 1 };
    let mask: u8 = ((1 << length) - 1) << mask_shift;
    byte &= mask as u8;
    byte >>= mask_shift;
    byte
}

/// set bit n in byte
pub fn set_bit(byte: &mut u8, n: u8, enable: bool) {
    if enable {
        *byte |= 1_u8 << n;
    } else {
        *byte &= !(1_u8 << n);
    }
}

/// Fill bits bitstart-bitstart+length in byte with data
pub fn set_bits(byte: &mut u8, bit_start: u8, length: u8, mut data: u8) {
    /*
              010 value to write
         76543210 bit numbers
            xxx   args: bit_start=4, length=3
         00011100 mask byte
         10101111 original value (sample)
         10100011 original & ~mask
         10101011 masked | value
     */

    // without mask_shift, strange behavior occurs, wenn bit_start < length.
    // e.g. bit_start=2, length = 2
    // in SOME cases, you get an 'attempt to subtract with overflow' exception, when
    // bitstart - length + 1 = 0
    // therefore just "cut off" at 0 shift
    let mask_shift: u8 = if bit_start < length { 0 } else { bit_start - length + 1 };
    let mask: u8 = ((1 << length) - 1) << mask_shift;
    data <<= mask_shift;                // shift data into correct position
    data &= mask;                       // zero all non-important bits in data
    *byte &= !(mask);                   // zero all important bits in existing byte
    *byte |= data;                      // combine data with existing byte
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate std;
    use std::*;

    use crate::device::*;

    #[test]
    fn get_bit_test() {
        assert_eq!(get_bit(4, 2), 1);
        assert_eq!(get_bit(4, 1), 0);
        assert_eq!(get_bit(4, 0), 0);
        assert_eq!(get_bit(12, 3), 1);
        assert_eq!(get_bit(12, 2), 1);
        assert_eq!(get_bit(12, 1), 0);
        assert_eq!(get_bit(12, 1), 0);
        assert_eq!(get_bit(8, 3), 1);
        assert_eq!(get_bit(8, 2), 0);
        assert_eq!(get_bit(8, 1), 0);
        assert_eq!(get_bit(8, 0), 0);
    }

    #[test]
    fn set_bit_test() {
        let mut byte = 4_u8.to_be_bytes();

        // enable bit 1
        set_bit(&mut byte[0], 1, true);
        assert_eq!(byte[0], 6);

        // disable bit 1
        set_bit(&mut byte[0], 1, false);
        assert_eq!(byte[0], 4);

        // enable bit 3
        set_bit(&mut byte[0], 3, true);
        assert_eq!(byte[0], 12);

        byte = ((1 << 7) as u8).to_be_bytes();
        set_bit(&mut byte[0], 7, false);
        assert_eq!(byte[0], 0);

        set_bit(&mut byte[0], 7, true);
        assert_eq!(byte[0], (1 << 7) as u8);
    }

    #[test]
    fn set_get_bits_test() {
        // 010 value to write
        // 76543210 bit numbers
        // xxx   args: bit_start=4, length=3
        // 00011100 mask byte
        // 10101111 original value (sample)
        // 10100011 original & ~mask
        // 10101011 masked | value
        let mut original_value: u8 = 175;
        let value: u8 = 2;
        let bitstart: u8 = 4;
        let length: u8 = 3;
        set_bits(&mut original_value, bitstart, length, value);
        assert_eq!(original_value, 0b10101011);

        let bits = get_bits(original_value, bitstart, length);
        assert_eq!(value, bits);

        // simulate accel_hpf
        let bitstart = Bits::ACCEL_CONFIG_ACCEL_HPF_BIT;
        let length = Bits::ACCEL_CONFIG_ACCEL_HPF_LENGTH;
        assert_eq!(get_bits(original_value, bitstart, length), 0b00000011);

        let mode: u8 = 7;
        set_bits(&mut original_value, bitstart, length, mode);
        assert_eq!(get_bits(original_value, bitstart, length), 0b00000111);
    }
}
