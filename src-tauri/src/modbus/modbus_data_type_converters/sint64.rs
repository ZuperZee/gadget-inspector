pub fn vec_uint64_to_sint64(u: &Vec<u64>) -> Vec<i64> {
    let vec: Vec<i64> = u.iter().map(|x| *x as i64).collect();
    return vec;
}

#[cfg(test)]
mod tests {
    use crate::modbus::modbus_data_type_converters::sint64::vec_uint64_to_sint64;

    #[test]
    fn vec_uint64_converts_to_sint64() {
        assert_eq!(vec_uint64_to_sint64(&vec![u64::MIN]), vec![0]); // Min input
        assert_eq!(vec_uint64_to_sint64(&vec![u64::MAX]), vec![-1]); // Max input
        assert_eq!(
            vec_uint64_to_sint64(&vec![0x7fff_ffff_ffff_ffff]),
            vec![i64::MAX]
        ); // Max positive output
        assert_eq!(
            vec_uint64_to_sint64(&vec![0x8000_0000_0000_0000]),
            vec![i64::MIN]
        ); // Max negative output

        // Single
        assert_eq!(vec_uint64_to_sint64(&vec![0xff]), vec![255]);
        assert_eq!(
            vec_uint64_to_sint64(&vec![0xffff_ffff_ffff_ff00]),
            vec![-256]
        );

        // Multiple
        assert_eq!(
            vec_uint64_to_sint64(&vec![0x32, 0xffff_ffff_ffff_cf08, 0x384]),
            vec![50, -12536, 900]
        );
    }
}
