pub fn vec_uint16_to_sint16(u: &[u16]) -> Vec<i16> {
    u.iter().map(|x| *x as i16).collect()
}

#[cfg(test)]
mod tests {
    use super::vec_uint16_to_sint16;

    #[test]
    fn vec_uint16_converts_to_sint16() {
        assert_eq!(vec_uint16_to_sint16(&[0]), vec![0]); // Min
        assert_eq!(vec_uint16_to_sint16(&[65535]), vec![-1]); // Max
        assert_eq!(vec_uint16_to_sint16(&[32767]), vec![32767]); // Max positive
        assert_eq!(vec_uint16_to_sint16(&[32768]), vec![-32768]); // Max negative

        // Single
        assert_eq!(vec_uint16_to_sint16(&[100]), vec![100]);
        assert_eq!(vec_uint16_to_sint16(&[50000]), vec![-15536]);

        // Multiple
        assert_eq!(
            vec_uint16_to_sint16(&[50, 53000, 900, 62000, 36000]),
            vec![50, -12536, 900, -3536, -29536]
        );
    }
}
