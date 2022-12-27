pub fn vec_uint32_to_sint32(u: &Vec<u32>) -> Vec<i32> {
    let vec: Vec<i32> = u.iter().map(|x| *x as i32).collect();
    return vec;
}

#[cfg(test)]
mod tests {
    use crate::modbus::modbus_data_type_converters::sint32::vec_uint32_to_sint32;

    #[test]
    fn vec_uint32_converts_to_sint32() {
        assert_eq!(vec_uint32_to_sint32(&vec![u32::MIN]), vec![0]); // Min input
        assert_eq!(vec_uint32_to_sint32(&vec![u32::MAX]), vec![-1]); // Max input
        assert_eq!(vec_uint32_to_sint32(&vec![0x7fff_ffff]), vec![i32::MAX]); // Max positive output
        assert_eq!(vec_uint32_to_sint32(&vec![0x8000_0000]), vec![i32::MIN]); // Max negative output

        // Single
        assert_eq!(vec_uint32_to_sint32(&vec![0xff]), vec![255]);
        assert_eq!(vec_uint32_to_sint32(&vec![0xffff_ff00]), vec![-256]);

        // Multiple
        assert_eq!(
            vec_uint32_to_sint32(&vec![0x32, 0xffff_cf08, 0x384]),
            vec![50, -12536, 900]
        );
    }
}
