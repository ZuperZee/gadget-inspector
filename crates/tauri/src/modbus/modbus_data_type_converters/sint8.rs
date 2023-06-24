pub fn vec_uint8_to_sint8(u: &[u8]) -> Vec<i8> {
    u.iter().map(|x| *x as i8).collect()
}

#[cfg(test)]
mod tests {
    use super::vec_uint8_to_sint8;

    #[test]
    fn vec_uint8_converts_to_sint8() {
        assert_eq!(vec_uint8_to_sint8(&[0]), vec![0]); // Min
        assert_eq!(vec_uint8_to_sint8(&[255]), vec![-1]); // Max
        assert_eq!(vec_uint8_to_sint8(&[127]), vec![127]); // Max positive
        assert_eq!(vec_uint8_to_sint8(&[128]), vec![-128]); // Max negative

        // Single
        assert_eq!(vec_uint8_to_sint8(&[100]), vec![100]);
        assert_eq!(vec_uint8_to_sint8(&[200]), vec![-56]);

        // Multiple
        assert_eq!(
            vec_uint8_to_sint8(&[50, 230, 90, 120, 160]),
            vec![50, -26, 90, 120, -96]
        );
    }
}
