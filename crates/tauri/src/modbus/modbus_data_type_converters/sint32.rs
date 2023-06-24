pub fn vec_uint32_to_sint32(u: &[u32]) -> Vec<i32> {
    u.iter().map(|x| *x as i32).collect()
}

#[cfg(test)]
mod tests {
    use super::vec_uint32_to_sint32;

    #[test]
    fn vec_uint32_converts_to_sint32() {
        assert_eq!(vec_uint32_to_sint32(&[u32::MIN]), vec![0]); // Min input
        assert_eq!(vec_uint32_to_sint32(&[u32::MAX]), vec![-1]); // Max input
        assert_eq!(vec_uint32_to_sint32(&[0x7fff_ffff]), vec![i32::MAX]); // Max positive output
        assert_eq!(vec_uint32_to_sint32(&[0x8000_0000]), vec![i32::MIN]); // Max negative output

        // Single
        assert_eq!(vec_uint32_to_sint32(&[0xff]), vec![255]);
        assert_eq!(vec_uint32_to_sint32(&[0xffff_ff00]), vec![-256]);

        // Multiple
        assert_eq!(
            vec_uint32_to_sint32(&[0x32, 0xffff_cf08, 0x384]),
            vec![50, -12536, 900]
        );
    }
}
