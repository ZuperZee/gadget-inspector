pub fn vec_uint8_to_uint16(u: &Vec<u8>) -> Vec<u16> {
    let u_len = u.len();
    let v_range = u_len - u_len % 2;

    let mut vec = Vec::new();
    for i in (0..v_range).step_by(2) {
        vec.push(u16::from_be_bytes([u[i], u[i + 1]]));
    }

    return vec;
}

#[cfg(test)]
mod tests {
    use crate::modbus_data_type_converters::uint16::vec_uint8_to_uint16;

    #[test]
    fn vec_uint8_converts_to_uint16() {
        assert_eq!(vec_uint8_to_uint16(&vec![0, 0]), vec![0]); // Min
        assert_eq!(vec_uint8_to_uint16(&vec![255, 255]), vec![65535]); // Max

        // Single
        assert_eq!(vec_uint8_to_uint16(&vec![0, 10]), vec![10]);
        assert_eq!(vec_uint8_to_uint16(&vec![26, 133]), vec![6789]);

        // Multiple
        assert_eq!(
            vec_uint8_to_uint16(&vec![48, 57, 26, 133]),
            vec![12345, 6789]
        );

        // Not full
        assert_eq!(vec_uint8_to_uint16(&vec![26, 133, 5]), vec![6789]);

        // Empty
        assert_eq!(vec_uint8_to_uint16(&vec![]), vec![] as Vec<u16>);
        assert_eq!(vec_uint8_to_uint16(&vec![100]), vec![] as Vec<u16>);
        assert_eq!(vec_uint8_to_uint16(&vec![0]), vec![] as Vec<u16>);
    }
}
