// [1,2,3,4,5,6,7,8] -> [[1,2,3,4] [3,4,5,6] [5,6,7,8]] -> [16909060, 50595078, 84281096]
pub fn vec_uint8_to_uint32(u: &Vec<u8>) -> Vec<u32> {
    let u_len = u.len();
    let v_range = u_len - u_len % 2;

    let mut vec = Vec::new();
    let mut i = 0;
    // Needs 4 elements in the range to create a u32 (+2 makes sure it's always 4 elements)
    while i + 2 < v_range {
        vec.push(u32::from_be_bytes([u[i], u[i + 1], u[i + 2], u[i + 3]]));
        i += 2;
    }

    return vec;
}

#[cfg(test)]
mod tests {
    use crate::modbus_data_type_converters::uint32::vec_uint8_to_uint32;

    #[test]
    fn t() {
        assert_eq!(
            vec_uint8_to_uint32(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
            vec![16909060, 50595078, 84281096]
        );
    }

    #[test]
    fn vec_uint8_converts_to_uint32() {
        assert_eq!(vec_uint8_to_uint32(&vec![0, 0, 0, 0]), vec![0]); // Min
        assert_eq!(
            vec_uint8_to_uint32(&vec![255, 255, 255, 255]),
            vec![4294967295]
        ); // Max

        // Single
        assert_eq!(vec_uint8_to_uint32(&vec![0, 0, 0, 10]), vec![10]);
        assert_eq!(vec_uint8_to_uint32(&vec![3, 4, 5, 6]), vec![50595078]);

        // Multiple
        assert_eq!(
            vec_uint8_to_uint32(&vec![0, 0, 0, 10, 0, 0, 0, 20]),
            vec![10, 655360, 20]
        );
        assert_eq!(
            vec_uint8_to_uint32(&vec![1, 2, 3, 4, 5, 6, 7, 8]),
            vec![16909060, 50595078, 84281096]
        );
        assert_eq!(
            vec_uint8_to_uint32(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
            vec![16909060, 50595078, 84281096, 117967114]
        );

        // Not full
        assert_eq!(
            vec_uint8_to_uint32(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9]),
            vec![16909060, 50595078, 84281096]
        );

        // Empty
        assert_eq!(vec_uint8_to_uint32(&vec![]), vec![] as Vec<u32>);
        assert_eq!(vec_uint8_to_uint32(&vec![100]), vec![] as Vec<u32>);
        assert_eq!(vec_uint8_to_uint32(&vec![100, 20]), vec![] as Vec<u32>);
        assert_eq!(vec_uint8_to_uint32(&vec![0, 45, 154]), vec![] as Vec<u32>);
        assert_eq!(vec_uint8_to_uint32(&vec![0, 0, 0]), vec![] as Vec<u32>);
    }
}