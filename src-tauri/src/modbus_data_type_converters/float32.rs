pub fn vec_uint8_to_float32(u: &Vec<u8>) -> Vec<f32> {
    let u_len = u.len();
    let v_range = u_len - u_len % 2;

    let mut vec = Vec::new();
    let mut i = 0;
    // Needs 4 elements in the range to create a f32 (+2 makes sure it's always 4 elements)
    while i + 2 < v_range {
        vec.push(f32::from_be_bytes([u[i], u[i + 1], u[i + 2], u[i + 3]]));
        i += 2;
    }

    return vec;
}

#[cfg(test)]
mod tests {
    use crate::modbus_data_type_converters::float32::vec_uint8_to_float32;

    #[test]
    fn vec_uint8_converts_to_float32() {
        // https://en.wikipedia.org/wiki/Single-precision_floating-point_format
        assert_eq!(vec_uint8_to_float32(&vec![0, 0, 0, 0]), vec![0.0]); // 0
        assert_eq!(vec_uint8_to_float32(&vec![128, 0, 0, 0]), vec![-0.0]); // -0
        assert_eq!(vec_uint8_to_float32(&vec![192, 0, 0, 0]), vec![-2.0]); // -2
        assert_eq!(vec_uint8_to_float32(&vec![63, 128, 0, 0]), vec![1.0]); // 1
        assert_eq!(
            vec_uint8_to_float32(&vec![127, 128, 0, 0]),
            vec![f32::INFINITY]
        ); // Infinity
        assert_eq!(
            vec_uint8_to_float32(&vec![255, 128, 0, 0]),
            vec![f32::NEG_INFINITY]
        ); // -Infinity
        assert_eq!(
            vec_uint8_to_float32(&vec![64, 73, 15, 219]),
            vec![std::f32::consts::PI]
        ); // PI

        assert_eq!(
            vec_uint8_to_float32(&vec![0, 0, 0, 1]),
            vec![1.4012984643e-45]
        ); // Smallest positive subnormal number
        assert_eq!(
            vec_uint8_to_float32(&vec![0, 127, 255, 255]),
            vec![1.1754942107e-38]
        ); // Largest subnormal number
        assert_eq!(
            vec_uint8_to_float32(&vec![0, 128, 0, 0]),
            vec![1.1754943508e-38]
        ); // Smallest positive normal number
        assert_eq!(
            vec_uint8_to_float32(&vec![127, 127, 255, 255]),
            vec![3.4028234664e38]
        ); // Largest normal number
        assert_eq!(
            vec_uint8_to_float32(&vec![63, 127, 255, 255]),
            vec![0.999999940395355225]
        ); // Largest number less than one
        assert_eq!(
            vec_uint8_to_float32(&vec![63, 128, 0, 1]),
            vec![1.00000011920928955]
        ); // Smallest number larger than one

        // Single
        assert_eq!(vec_uint8_to_float32(&vec![0, 0, 0, 10]), vec![1.4e-44]);

        // Empty
        assert_eq!(vec_uint8_to_float32(&vec![]), vec![] as Vec<f32>);
        assert_eq!(vec_uint8_to_float32(&vec![100]), vec![] as Vec<f32>);
        assert_eq!(vec_uint8_to_float32(&vec![100, 20]), vec![] as Vec<f32>);
        assert_eq!(vec_uint8_to_float32(&vec![0, 45, 154]), vec![] as Vec<f32>);
        assert_eq!(vec_uint8_to_float32(&vec![0, 0, 0]), vec![] as Vec<f32>);
    }
}
