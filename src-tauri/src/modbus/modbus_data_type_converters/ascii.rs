pub fn vec_uint8_to_ascii(u: &[u8]) -> Vec<char> {
    u.iter().map(|x| *x as char).collect()
}

#[cfg(test)]
mod tests {
    use super::vec_uint8_to_ascii;

    #[test]
    fn vec_uint8_converts_to_sint8() {
        assert_eq!(vec_uint8_to_ascii(&[32]), vec![' ']); // Space
        assert_eq!(vec_uint8_to_ascii(&[10]), vec!['\n']); // LF (Line Feed, end of line)
        assert_eq!(vec_uint8_to_ascii(&[65]), vec!['A']);
        assert_eq!(vec_uint8_to_ascii(&[66]), vec!['B']);
        assert_eq!(vec_uint8_to_ascii(&[67]), vec!['C']);
        assert_eq!(vec_uint8_to_ascii(&[90]), vec!['Z']);

        assert_eq!(vec_uint8_to_ascii(&[97]), vec!['a']);
        assert_eq!(vec_uint8_to_ascii(&[98]), vec!['b']);
        assert_eq!(vec_uint8_to_ascii(&[99]), vec!['c']);
        assert_eq!(vec_uint8_to_ascii(&[122]), vec!['z']);

        assert_eq!(vec_uint8_to_ascii(&[91]), vec!['[']);

        assert_eq!(vec_uint8_to_ascii(&[197]), vec!['Å']);
    }
}
