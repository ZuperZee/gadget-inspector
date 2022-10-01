// [1,2,3,4,5,6,7,8] -> [[1,2,3,4] [3,4,5,6] [5,6,7,8]] -> [16909060, 50595078, 84281096]
pub fn vec_uint8_to_uint64(u: &Vec<u8>) -> Vec<u64> {
    let u_len = u.len();
    let v_range = u_len - u_len % 2;

    let mut vec = Vec::new();
    let mut i = 0;
    // Needs 8 elements in the range to create a u64 (+6 makes sure it's always 8 elements)
    while i + 6 < v_range {
        vec.push(u64::from_be_bytes([
            u[i],
            u[i + 1],
            u[i + 2],
            u[i + 3],
            u[i + 4],
            u[i + 5],
            u[i + 6],
            u[i + 7],
        ]));
        i += 2;
    }

    return vec;
}

#[cfg(test)]
mod tests {
    use crate::modbus_data_type_converters::uint64::vec_uint8_to_uint64;

    #[test]
    fn vec_uint8_converts_to_uint32() {
        assert_eq!(vec_uint8_to_uint64(&vec![0, 0, 0, 0, 0, 0, 0, 0]), vec![0]); // Min
        assert_eq!(
            vec_uint8_to_uint64(&vec![255, 255, 255, 255, 255, 255, 255, 255]),
            vec![18446744073709551615]
        ); // Max

        // Single
        assert_eq!(
            vec_uint8_to_uint64(&vec![0, 0, 0, 0, 0, 0, 0, 10]),
            vec![10]
        );
        // assert_eq!(vec_uint8_to_uint64(&vec![3, 4, 5, 6]), vec![50595078]);

        // Multiple
        assert_eq!(
            vec_uint8_to_uint64(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]),
            vec![
                72623859790382856,
                217304205466536202,
                361984551142689548,
                506664896818842894,
                651345242494996240,
            ]
        );
        assert_eq!(
            vec_uint8_to_uint64(&vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20
            ]),
            vec![
                72623859790382856,
                217304205466536202,
                361984551142689548,
                506664896818842894,
                651345242494996240,
                796025588171149586,
                940705933847302932
            ]
        );
        assert_eq!(
            vec_uint8_to_uint64(&vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 1, 2, 3, 4,
                5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20
            ]),
            vec![
                72623859790382856,
                217304205466536202,
                361984551142689548,
                506664896818842894,
                651345242494996240,
                796025588171149586,
                940705933847302932,
                1085386279523451138,
                1230066624862749444,
                1374724894407001350,
                72623859790382856,
                217304205466536202,
                361984551142689548,
                506664896818842894,
                651345242494996240,
                796025588171149586,
                940705933847302932
            ]
        );

        // Not full
        assert_eq!(
            vec_uint8_to_uint64(&vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17
            ]),
            vec![
                72623859790382856,
                217304205466536202,
                361984551142689548,
                506664896818842894,
                651345242494996240,
            ]
        );

        // Empty
        assert_eq!(vec_uint8_to_uint64(&vec![]), vec![] as Vec<u64>);
        assert_eq!(vec_uint8_to_uint64(&vec![100]), vec![] as Vec<u64>);
        assert_eq!(vec_uint8_to_uint64(&vec![100, 20]), vec![] as Vec<u64>);
        assert_eq!(vec_uint8_to_uint64(&vec![0, 45, 154]), vec![] as Vec<u64>);
        assert_eq!(vec_uint8_to_uint64(&vec![0, 0, 0]), vec![] as Vec<u64>);
        assert_eq!(vec_uint8_to_uint64(&vec![0, 0, 0, 12]), vec![] as Vec<u64>);
        assert_eq!(
            vec_uint8_to_uint64(&vec![0, 0, 0, 12, 2]),
            vec![] as Vec<u64>
        );
        assert_eq!(
            vec_uint8_to_uint64(&vec![0, 0, 0, 12, 2, 15, 1]),
            vec![] as Vec<u64>
        );
    }
}
