pub fn vec_uint8_to_float64(u: &[u8], is_word_swap: bool) -> Vec<f64> {
    let u_len = u.len();
    let v_range = u_len - u_len % 2;

    let mut vec = Vec::new();
    let mut i = 0;
    // Needs 8 elements in the range to create a f64 (+6 makes sure it's always 8 elements)
    while i + 6 < v_range {
        match is_word_swap {
            false => vec.push(f64::from_be_bytes([
                u[i],
                u[i + 1],
                u[i + 2],
                u[i + 3],
                u[i + 4],
                u[i + 5],
                u[i + 6],
                u[i + 7],
            ])),
            true => vec.push(f64::from_be_bytes([
                u[i + 2],
                u[i + 3],
                u[i],
                u[i + 1],
                u[i + 6],
                u[i + 7],
                u[i + 4],
                u[i + 5],
            ])),
        }
        i += 2;
    }

    vec
}

#[cfg(test)]
mod tests {
    use super::vec_uint8_to_float64;

    #[test]
    fn vec_uint8_converts_to_float64() {
        // https://en.wikipedia.org/wiki/Double-precision_floating-point_format
        assert_eq!(
            vec_uint8_to_float64(&[0, 0, 0, 0, 0, 0, 0, 0], false),
            vec![0.0]
        ); // 0
        assert_eq!(
            vec_uint8_to_float64(&[128, 0, 0, 0, 0, 0, 0, 0], false),
            vec![-0.0]
        ); // -0
        assert_eq!(
            vec_uint8_to_float64(&[192, 0, 0, 0, 0, 0, 0, 0], false),
            vec![-2.0]
        ); // -2
        assert_eq!(
            vec_uint8_to_float64(&[63, 240, 0, 0, 0, 0, 0, 0], false),
            vec![1.0]
        ); // 1
        assert_eq!(
            vec_uint8_to_float64(&[127, 240, 0, 0, 0, 0, 0, 0], false),
            vec![f64::INFINITY]
        ); // Infinity
        assert_eq!(
            vec_uint8_to_float64(&[255, 240, 0, 0, 0, 0, 0, 0], false),
            vec![f64::NEG_INFINITY]
        ); // -Infinity
        assert_eq!(
            vec_uint8_to_float64(&[64, 9, 33, 251, 84, 68, 45, 24], false),
            vec![std::f64::consts::PI]
        ); // PI

        assert_eq!(
            vec_uint8_to_float64(&[0, 0, 0, 0, 0, 0, 0, 1], false),
            vec![5e-324]
        ); // Smallest positive subnormal number
        assert_eq!(
            vec_uint8_to_float64(&[0, 15, 255, 255, 255, 255, 255, 255], false),
            vec![2.225_073_858_507_201e-308]
        ); // Largest subnormal number
        assert_eq!(
            vec_uint8_to_float64(&[0, 16, 0, 0, 0, 0, 0, 0], false),
            vec![2.2250738585072014e-308]
        ); // Smallest positive normal number
        assert_eq!(
            vec_uint8_to_float64(&[127, 239, 255, 255, 255, 255, 255, 255], false),
            vec![1.7976931348623157e308]
        ); // Largest normal number
        assert_eq!(
            vec_uint8_to_float64(&[63, 240, 0, 0, 0, 0, 0, 1], false),
            vec![1.0000000000000002]
        ); // Smallest number larger than one

        // Empty
        assert_eq!(vec_uint8_to_float64(&[], false), vec![] as Vec<f64>);
        assert_eq!(vec_uint8_to_float64(&[100], false), vec![] as Vec<f64>);
        assert_eq!(vec_uint8_to_float64(&[100, 20], false), vec![] as Vec<f64>);
        assert_eq!(
            vec_uint8_to_float64(&[0, 45, 154], false),
            vec![] as Vec<f64>
        );
        assert_eq!(vec_uint8_to_float64(&[0, 0, 0], false), vec![] as Vec<f64>);
        assert_eq!(
            vec_uint8_to_float64(&[0, 0, 0, 12], false),
            vec![] as Vec<f64>
        );
        assert_eq!(
            vec_uint8_to_float64(&[0, 0, 0, 12, 2], false),
            vec![] as Vec<f64>
        );
        assert_eq!(
            vec_uint8_to_float64(&[0, 0, 0, 12, 2, 15, 1], false),
            vec![] as Vec<f64>
        );
    }

    #[test]
    fn vec_uint8_converts_to_float64_word_swapped() {
        // https://en.wikipedia.org/wiki/Double-precision_floating-point_format
        assert_eq!(
            vec_uint8_to_float64(&[0, 0, 0, 0, 0, 0, 0, 0], true),
            vec![0.0]
        ); // 0
        assert_eq!(
            vec_uint8_to_float64(&[0, 0, 128, 0, 0, 0, 0, 0], true),
            vec![-0.0]
        ); // -0
        assert_eq!(
            vec_uint8_to_float64(&[0, 0, 192, 0, 0, 0, 0, 0], true),
            vec![-2.0]
        ); // -2
        assert_eq!(
            vec_uint8_to_float64(&[0, 0, 63, 240, 0, 0, 0, 0], true),
            vec![1.0]
        ); // 1
        assert_eq!(
            vec_uint8_to_float64(&[0, 0, 127, 240, 0, 0, 0, 0], true),
            vec![f64::INFINITY]
        ); // Infinity
        assert_eq!(
            vec_uint8_to_float64(&[0, 0, 255, 240, 0, 0, 0, 0], true),
            vec![f64::NEG_INFINITY]
        ); // -Infinity
        assert_eq!(
            vec_uint8_to_float64(&[33, 251, 64, 9, 45, 24, 84, 68], true),
            vec![std::f64::consts::PI]
        ); // PI

        assert_eq!(
            vec_uint8_to_float64(&[0, 0, 0, 0, 0, 1, 0, 0], true),
            vec![5e-324]
        ); // Smallest positive subnormal number
        assert_eq!(
            vec_uint8_to_float64(&[255, 255, 0, 15, 255, 255, 255, 255], true),
            vec![2.225_073_858_507_201e-308]
        ); // Largest subnormal number
        assert_eq!(
            vec_uint8_to_float64(&[0, 0, 0, 16, 0, 0, 0, 0], true),
            vec![2.2250738585072014e-308]
        ); // Smallest positive normal number
        assert_eq!(
            vec_uint8_to_float64(&[255, 255, 127, 239, 255, 255, 255, 255], true),
            vec![1.7976931348623157e308]
        ); // Largest normal number
        assert_eq!(
            vec_uint8_to_float64(&[0, 0, 63, 240, 0, 1, 0, 0], true),
            vec![1.0000000000000002]
        ); // Smallest number larger than one

        // Empty
        assert_eq!(vec_uint8_to_float64(&[], true), vec![] as Vec<f64>);
        assert_eq!(vec_uint8_to_float64(&[100], true), vec![] as Vec<f64>);
        assert_eq!(vec_uint8_to_float64(&[100, 20], true), vec![] as Vec<f64>);
        assert_eq!(
            vec_uint8_to_float64(&[0, 45, 154], true),
            vec![] as Vec<f64>
        );
        assert_eq!(vec_uint8_to_float64(&[0, 0, 0], true), vec![] as Vec<f64>);
        assert_eq!(
            vec_uint8_to_float64(&[0, 0, 0, 12], true),
            vec![] as Vec<f64>
        );
        assert_eq!(
            vec_uint8_to_float64(&[0, 0, 0, 12, 2], true),
            vec![] as Vec<f64>
        );
        assert_eq!(
            vec_uint8_to_float64(&[0, 0, 0, 12, 2, 15, 1], true),
            vec![] as Vec<f64>
        );
    }
}
