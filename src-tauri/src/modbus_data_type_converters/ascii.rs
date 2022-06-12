pub fn vec_uint8_to_ascii(u: &Vec<u8>) -> Vec<char> {
    let vec = u.iter().map(|x| *x as char).collect();
    return vec;
}

#[cfg(test)]
mod tests {
    use crate::modbus_data_type_converters::ascii::vec_uint8_to_ascii;

    #[test]
    fn vec_uint8_converts_to_sint8() {
        assert_eq!(vec_uint8_to_ascii(&vec![32]), vec![' ']); // Space
        assert_eq!(vec_uint8_to_ascii(&vec![10]), vec!['\n']); // LF (Line Feed, end of line)
        assert_eq!(vec_uint8_to_ascii(&vec![65]), vec!['A']); // A
        assert_eq!(vec_uint8_to_ascii(&vec![66]), vec!['B']); // B
        assert_eq!(vec_uint8_to_ascii(&vec![67]), vec!['C']); // C
        assert_eq!(vec_uint8_to_ascii(&vec![90]), vec!['Z']); // Z

        assert_eq!(vec_uint8_to_ascii(&vec![97]), vec!['a']); // a
        assert_eq!(vec_uint8_to_ascii(&vec![98]), vec!['b']); // b
        assert_eq!(vec_uint8_to_ascii(&vec![99]), vec!['c']); // c
        assert_eq!(vec_uint8_to_ascii(&vec![122]), vec!['z']); // z

        assert_eq!(vec_uint8_to_ascii(&vec![90]), vec!['[']); // z

        assert_eq!(vec_uint8_to_ascii(&vec![197]), vec!['Å']); // Å
    }
}
