// [1,2,3,4,5,6,7,8] -> [[1,2,3,4] [3,4,5,6] [5,6,7,8]] -> [16909060, 50595078, 84281096]
// [1,2,3,4,5,6,7,8] -> [[3,4,1,2] [5,6,3,4] [7,8,5,6]] -> [50594050, 84280068, 117966086] | word swap
pub fn vec_uint8_to_uint32(u: &[u8], is_word_swap: bool) -> Vec<u32> {
    let u_len = u.len();
    let v_range = u_len - u_len % 2;

    let mut vec = Vec::new();
    let mut i = 0;
    // Needs 4 elements in the range to create a u32 (+2 makes sure it's always 4 elements)
    while i + 2 < v_range {
        match is_word_swap {
            false => vec.push(u32::from_be_bytes([u[i], u[i + 1], u[i + 2], u[i + 3]])),
            true => vec.push(u32::from_be_bytes([u[i + 2], u[i + 3], u[i], u[i + 1]])),
        }
        i += 2;
    }

    vec
}

#[cfg(test)]
mod tests {
    use super::vec_uint8_to_uint32;

    #[test]
    fn vec_uint8_converts_to_uint32() {
        assert_eq!(vec_uint8_to_uint32(&[0, 0, 0, 0], false), vec![0]); // Min
        assert_eq!(
            vec_uint8_to_uint32(&[255, 255, 255, 255], false),
            vec![4294967295]
        ); // Max

        // Single
        assert_eq!(vec_uint8_to_uint32(&[0, 0, 0, 10], false), vec![10]);
        assert_eq!(vec_uint8_to_uint32(&[3, 4, 5, 6], false), vec![50595078]);

        // Multiple
        assert_eq!(
            vec_uint8_to_uint32(&[0, 0, 0, 10, 0, 0, 0, 20], false),
            vec![10, 655360, 20]
        );
        assert_eq!(
            vec_uint8_to_uint32(&[1, 2, 3, 4, 5, 6, 7, 8], false),
            vec![16909060, 50595078, 84281096]
        );
        assert_eq!(
            vec_uint8_to_uint32(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], false),
            vec![16909060, 50595078, 84281096, 117967114]
        );

        // Not full
        assert_eq!(
            vec_uint8_to_uint32(&[1, 2, 3, 4, 5, 6, 7, 8, 9], false),
            vec![16909060, 50595078, 84281096]
        );

        // Empty
        assert_eq!(vec_uint8_to_uint32(&[], false), vec![] as Vec<u32>);
        assert_eq!(vec_uint8_to_uint32(&[100], false), vec![] as Vec<u32>);
        assert_eq!(vec_uint8_to_uint32(&[100, 20], false), vec![] as Vec<u32>);
        assert_eq!(
            vec_uint8_to_uint32(&[0, 45, 154], false),
            vec![] as Vec<u32>
        );
        assert_eq!(vec_uint8_to_uint32(&[0, 0, 0], false), vec![] as Vec<u32>);
    }

    #[test]
    fn vec_uint8_converts_to_uint32_word_swapped() {
        assert_eq!(vec_uint8_to_uint32(&[0, 0, 0, 0], true), vec![0]); // Min
        assert_eq!(
            vec_uint8_to_uint32(&[255, 255, 255, 255], true),
            vec![4294967295]
        ); // Max

        // Single
        assert_eq!(vec_uint8_to_uint32(&[0, 10, 0, 0], true), vec![10]);
        assert_eq!(vec_uint8_to_uint32(&[5, 6, 3, 4], true), vec![50595078]);
        assert_eq!(vec_uint8_to_uint32(&[0, 0, 0, 10], true), vec![655360]);
        assert_eq!(vec_uint8_to_uint32(&[3, 4, 5, 6], true), vec![84280068]);

        // Multiple
        assert_eq!(
            vec_uint8_to_uint32(&[0, 10, 0, 0, 0, 20, 0, 0], true),
            vec![10, 1310720, 20]
        );
        assert_eq!(
            vec_uint8_to_uint32(&[1, 2, 3, 4, 5, 6, 7, 8], true),
            vec![50594050, 84280068, 117966086]
        );
        assert_eq!(
            vec_uint8_to_uint32(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], true),
            vec![50594050, 84280068, 117966086, 151652104]
        );

        // Not full
        assert_eq!(
            vec_uint8_to_uint32(&[1, 2, 3, 4, 5, 6, 7, 8, 9], true),
            vec![50594050, 84280068, 117966086]
        );

        // Empty
        assert_eq!(vec_uint8_to_uint32(&[], true), vec![] as Vec<u32>);
        assert_eq!(vec_uint8_to_uint32(&[100], true), vec![] as Vec<u32>);
        assert_eq!(vec_uint8_to_uint32(&[100, 20], true), vec![] as Vec<u32>);
        assert_eq!(vec_uint8_to_uint32(&[0, 45, 154], true), vec![] as Vec<u32>);
        assert_eq!(vec_uint8_to_uint32(&[0, 0, 0], true), vec![] as Vec<u32>);
    }
}
