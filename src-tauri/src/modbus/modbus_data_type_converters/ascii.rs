pub fn vec_uint8_to_ascii(u: &Vec<u8>) -> Vec<char> {
    let vec = u.iter().map(|x| *x as char).collect();
    return vec;
}

#[cfg(test)]
mod tests {
    use crate::modbus::modbus_data_type_converters::ascii::vec_uint8_to_ascii;

    #[test]
    fn vec_uint8_converts_to_sint8() {
        assert_eq!(vec_uint8_to_ascii(&vec![32]), vec![' ']); // Space
        assert_eq!(vec_uint8_to_ascii(&vec![10]), vec!['\n']); // LF (Line Feed, end of line)
        assert_eq!(vec_uint8_to_ascii(&vec![65]), vec!['A']);
        assert_eq!(vec_uint8_to_ascii(&vec![66]), vec!['B']);
        assert_eq!(vec_uint8_to_ascii(&vec![67]), vec!['C']);
        assert_eq!(vec_uint8_to_ascii(&vec![90]), vec!['Z']);

        assert_eq!(vec_uint8_to_ascii(&vec![97]), vec!['a']);
        assert_eq!(vec_uint8_to_ascii(&vec![98]), vec!['b']);
        assert_eq!(vec_uint8_to_ascii(&vec![99]), vec!['c']);
        assert_eq!(vec_uint8_to_ascii(&vec![122]), vec!['z']);

        assert_eq!(vec_uint8_to_ascii(&vec![91]), vec!['[']);

        assert_eq!(vec_uint8_to_ascii(&vec![197]), vec!['Å']);
    }
}
