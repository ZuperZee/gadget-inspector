pub fn vec_uint8_to_sint8(u: &Vec<u8>) -> Vec<i8> {
    let vec: Vec<i8> = u.iter().map(|x| *x as i8).collect();
    return vec;
}

#[cfg(test)]
mod tests {
    use crate::modbus::modbus_data_type_converters::sint8::vec_uint8_to_sint8;

    #[test]
    fn vec_uint8_converts_to_sint8() {
        assert_eq!(vec_uint8_to_sint8(&vec![0]), vec![0]); // Min
        assert_eq!(vec_uint8_to_sint8(&vec![255]), vec![-1]); // Max
        assert_eq!(vec_uint8_to_sint8(&vec![127]), vec![127]); // Max positive
        assert_eq!(vec_uint8_to_sint8(&vec![128]), vec![-128]); // Max negative

        // Single
        assert_eq!(vec_uint8_to_sint8(&vec![100]), vec![100]);
        assert_eq!(vec_uint8_to_sint8(&vec![200]), vec![-56]);

        // Multiple
        assert_eq!(
            vec_uint8_to_sint8(&vec![50, 230, 90, 120, 160]),
            vec![50, -26, 90, 120, -96]
        );
    }
}
