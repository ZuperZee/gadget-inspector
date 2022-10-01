pub fn vec_uint16_to_uint8(u: &Vec<u16>) -> Vec<u8> {
    let mut vec: Vec<u8> = Vec::new();

    for n in u {
        let high_byte: u8 = (n >> 8) as u8;
        let low_byte: u8 = (n & 0xff) as u8;

        vec.push(high_byte);
        vec.push(low_byte);
    }

    return vec;
}

#[cfg(test)]
mod tests {
    use crate::modbus_data_type_converters::uint8::vec_uint16_to_uint8;

    #[test]
    fn vec_uint16_converts_to_uint8() {
        assert_eq!(vec_uint16_to_uint8(&vec![0]), vec![0, 0]); // Min
        assert_eq!(vec_uint16_to_uint8(&vec![65535]), vec![255, 255]); // Max

        // Single
        assert_eq!(vec_uint16_to_uint8(&vec![10]), vec![0, 10]);
        assert_eq!(vec_uint16_to_uint8(&vec![6789]), vec![26, 133]);

        // Multiple
        assert_eq!(
            vec_uint16_to_uint8(&vec![12345, 6789]),
            vec![48, 57, 26, 133]
        );

        // Empty
        assert_eq!(vec_uint16_to_uint8(&vec![]), vec![] as Vec<u8>);
    }
}
