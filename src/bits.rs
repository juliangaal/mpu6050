// Mostly taken from https://github.com/jrowberg/i2cdevlib/blob/master/Arduino/I2Cdev/I2Cdev.cpp
// and tested

pub fn get_bit(byte: u8, n: u8) -> u8 {
    (byte >> n) & 1
}

pub fn get_bits(mut byte: u8, bit_start: u8, length: u8) -> u8 {
    // 01101001 read byte
    // 76543210 bit numbers
    //    xxx   args: bit_start=4, length=3
    //    010   masked
    //   -> 010 shifted
    let mask: u8 = ((1 << length) - 1) << (bit_start - length + 1);
    byte &= mask;
    byte >>= bit_start - length + 1;
    byte
}

pub fn set_bit(byte: &mut u8, n: u8, enable: bool) {
    if enable {
        *byte |= 1_u8 << n;
    } else {
        *byte &= !(1_u8 << n);
    }
}

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
    let mask: u8 = ((1 << length) - 1) << (bit_start - length + 1);
    data <<= bit_start - length + 1;    // shift data into correct position
    data &= mask;                       // zero all non-important bits in data
    *byte &= !(mask);                   // zero all important bits in existing byte
    *byte |= data;                      // combine data with existing byte
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_bit_test() {
        assert_eq!(get_bit(4, 2), 1);
        assert_eq!(get_bit(4, 1), 0);
        assert_eq!(get_bit(4, 0), 0);
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
    }
}
