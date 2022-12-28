pub fn vec_uint8_to_uint64(u: &[u8], is_word_swap: bool) -> Vec<u64> {
    let u_len = u.len();
    let v_range = u_len - u_len % 2;

    let mut vec = Vec::new();
    let mut i = 0;
    // Needs 8 elements in the range to create a u64 (+6 makes sure it's always 8 elements)
    while i + 6 < v_range {
        match is_word_swap {
            false => vec.push(u64::from_be_bytes([
                u[i],
                u[i + 1],
                u[i + 2],
                u[i + 3],
                u[i + 4],
                u[i + 5],
                u[i + 6],
                u[i + 7],
            ])),
            true => vec.push(u64::from_be_bytes([
                u[i + 2],
                u[i + 3],
                u[i],
                u[i + 1],
                u[i + 6],
                u[i + 7],
                u[i + 4],
                u[i + 5],
            ])),
        }
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

    vec
}

#[cfg(test)]
mod tests {
    use crate::modbus::modbus_data_type_converters::uint64::vec_uint8_to_uint64;

    #[test]
    fn vec_uint8_converts_to_uint32() {
        assert_eq!(
            vec_uint8_to_uint64(&[0, 0, 0, 0, 0, 0, 0, 0], false),
            vec![0]
        ); // Min
        assert_eq!(
            vec_uint8_to_uint64(&[255, 255, 255, 255, 255, 255, 255, 255], false),
            vec![18446744073709551615]
        ); // Max

        // Single
        assert_eq!(
            vec_uint8_to_uint64(&[0, 0, 0, 0, 0, 0, 0, 10], false),
            vec![10]
        );
        // assert_eq!(vec_uint8_to_uint64(&[3, 4, 5, 6], false), vec![50595078]);

        // Multiple
        assert_eq!(
            vec_uint8_to_uint64(
                &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
                false
            ),
            vec![
                72623859790382856,
                217304205466536202,
                361984551142689548,
                506664896818842894,
                651345242494996240,
            ]
        );
        assert_eq!(
            vec_uint8_to_uint64(
                &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20],
                false
            ),
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
            vec_uint8_to_uint64(
                &[
                    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 1, 2, 3,
                    4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20
                ],
                false
            ),
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
            vec_uint8_to_uint64(
                &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17],
                false
            ),
            vec![
                72623859790382856,
                217304205466536202,
                361984551142689548,
                506664896818842894,
                651345242494996240,
            ]
        );

        // Empty
        assert_eq!(vec_uint8_to_uint64(&[], false), vec![] as Vec<u64>);
        assert_eq!(vec_uint8_to_uint64(&[100], false), vec![] as Vec<u64>);
        assert_eq!(vec_uint8_to_uint64(&[100, 20], false), vec![] as Vec<u64>);
        assert_eq!(
            vec_uint8_to_uint64(&[0, 45, 154], false),
            vec![] as Vec<u64>
        );
        assert_eq!(vec_uint8_to_uint64(&[0, 0, 0], false), vec![] as Vec<u64>);
        assert_eq!(
            vec_uint8_to_uint64(&[0, 0, 0, 12], false),
            vec![] as Vec<u64>
        );
        assert_eq!(
            vec_uint8_to_uint64(&[0, 0, 0, 12, 2], false),
            vec![] as Vec<u64>
        );
        assert_eq!(
            vec_uint8_to_uint64(&[0, 0, 0, 12, 2, 15, 1], false),
            vec![] as Vec<u64>
        );
    }
}
