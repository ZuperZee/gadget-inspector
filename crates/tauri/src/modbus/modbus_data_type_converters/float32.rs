pub fn vec_uint8_to_float32(u: &[u8], is_word_swap: bool) -> Vec<f32> {
    let u_len = u.len();
    let v_range = u_len - u_len % 2;

    let mut vec = Vec::new();
    let mut i = 0;
    // Needs 4 elements in the range to create a f32 (+2 makes sure it's always 4 elements)
    while i + 2 < v_range {
        match is_word_swap {
            false => vec.push(f32::from_be_bytes([u[i], u[i + 1], u[i + 2], u[i + 3]])),
            true => vec.push(f32::from_be_bytes([u[i + 2], u[i + 3], u[i], u[i + 1]])),
        }
        i += 2;
    }

    vec
}

#[cfg(test)]
mod tests {
    use super::vec_uint8_to_float32;

    #[test]
    fn vec_uint8_converts_to_float32() {
        // https://en.wikipedia.org/wiki/Single-precision_floating-point_format
        assert_eq!(vec_uint8_to_float32(&[0, 0, 0, 0], false), vec![0.0]); // 0
        assert_eq!(vec_uint8_to_float32(&[128, 0, 0, 0], false), vec![-0.0]); // -0
        assert_eq!(vec_uint8_to_float32(&[192, 0, 0, 0], false), vec![-2.0]); // -2
        assert_eq!(vec_uint8_to_float32(&[63, 128, 0, 0], false), vec![1.0]); // 1
        assert_eq!(
            vec_uint8_to_float32(&[127, 128, 0, 0], false),
            vec![f32::INFINITY]
        ); // Infinity
        assert_eq!(
            vec_uint8_to_float32(&[255, 128, 0, 0], false),
            vec![f32::NEG_INFINITY]
        ); // -Infinity
        assert_eq!(
            vec_uint8_to_float32(&[64, 73, 15, 219], false),
            vec![std::f32::consts::PI]
        ); // PI

        assert_eq!(vec_uint8_to_float32(&[0, 0, 0, 1], false), vec![1e-45]); // Smallest positive subnormal number
        assert_eq!(
            vec_uint8_to_float32(&[0, 127, 255, 255], false),
            vec![1.175_494_2e-38]
        ); // Largest subnormal number
        assert_eq!(
            vec_uint8_to_float32(&[0, 128, 0, 0], false),
            vec![1.175_494_4e-38]
        ); // Smallest positive normal number
        assert_eq!(
            vec_uint8_to_float32(&[127, 127, 255, 255], false),
            vec![3.402_823_5e38]
        ); // Largest normal number
        assert_eq!(
            vec_uint8_to_float32(&[63, 127, 255, 255], false),
            vec![0.999_999_94]
        ); // Largest number less than one
        assert_eq!(
            vec_uint8_to_float32(&[63, 128, 0, 1], false),
            vec![1.000_000_1]
        ); // Smallest number larger than one

        // Single
        assert_eq!(vec_uint8_to_float32(&[0, 0, 0, 10], false), vec![1.4e-44]);

        // Empty
        assert_eq!(vec_uint8_to_float32(&[], false), vec![] as Vec<f32>);
        assert_eq!(vec_uint8_to_float32(&[100], false), vec![] as Vec<f32>);
        assert_eq!(vec_uint8_to_float32(&[100, 20], false), vec![] as Vec<f32>);
        assert_eq!(
            vec_uint8_to_float32(&[0, 45, 154], false),
            vec![] as Vec<f32>
        );
        assert_eq!(vec_uint8_to_float32(&[0, 0, 0], false), vec![] as Vec<f32>);
    }

    #[test]
    fn vec_uint8_converts_to_float32_word_swapped() {
        // https://en.wikipedia.org/wiki/Single-precision_floating-point_format
        assert_eq!(vec_uint8_to_float32(&[0, 0, 0, 0], true), vec![0.0]); // 0
        assert_eq!(vec_uint8_to_float32(&[0, 0, 128, 0], true), vec![-0.0]); // -0
        assert_eq!(vec_uint8_to_float32(&[0, 0, 192, 0], true), vec![-2.0]); // -2
        assert_eq!(vec_uint8_to_float32(&[0, 0, 63, 128], true), vec![1.0]); // 1
        assert_eq!(
            vec_uint8_to_float32(&[0, 0, 127, 128], true),
            vec![f32::INFINITY]
        ); // Infinity
        assert_eq!(
            vec_uint8_to_float32(&[0, 0, 255, 128], true),
            vec![f32::NEG_INFINITY]
        ); // -Infinity
        assert_eq!(
            vec_uint8_to_float32(&[15, 219, 64, 73], true),
            vec![std::f32::consts::PI]
        ); // PI

        assert_eq!(vec_uint8_to_float32(&[0, 1, 0, 0], true), vec![1e-45]); // Smallest positive subnormal number
        assert_eq!(
            vec_uint8_to_float32(&[255, 255, 0, 127], true),
            vec![1.175_494_2e-38]
        ); // Largest subnormal number
        assert_eq!(
            vec_uint8_to_float32(&[0, 0, 0, 128], true),
            vec![1.175_494_4e-38]
        ); // Smallest positive normal number
        assert_eq!(
            vec_uint8_to_float32(&[255, 255, 127, 127], true),
            vec![3.402_823_5e38]
        ); // Largest normal number
        assert_eq!(
            vec_uint8_to_float32(&[255, 255, 63, 127], true),
            vec![0.999_999_94]
        ); // Largest number less than one
        assert_eq!(
            vec_uint8_to_float32(&[0, 1, 63, 128], true),
            vec![1.000_000_1]
        ); // Smallest number larger than one

        // Single
        assert_eq!(vec_uint8_to_float32(&[0, 10, 0, 0], true), vec![1.4e-44]);

        // Empty
        assert_eq!(vec_uint8_to_float32(&[], true), vec![] as Vec<f32>);
        assert_eq!(vec_uint8_to_float32(&[100], true), vec![] as Vec<f32>);
        assert_eq!(vec_uint8_to_float32(&[100, 20], true), vec![] as Vec<f32>);
        assert_eq!(
            vec_uint8_to_float32(&[0, 45, 154], true),
            vec![] as Vec<f32>
        );
        assert_eq!(vec_uint8_to_float32(&[0, 0, 0], true), vec![] as Vec<f32>);
    }
}
