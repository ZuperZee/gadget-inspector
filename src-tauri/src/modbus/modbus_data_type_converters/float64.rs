pub fn vec_uint8_to_float64(u: &Vec<u8>) -> Vec<f64> {
    let u_len = u.len();
    let v_range = u_len - u_len % 2;

    let mut vec = Vec::new();
    let mut i = 0;
    // Needs 8 elements in the range to create a f64 (+6 makes sure it's always 8 elements)
    while i + 6 < v_range {
        vec.push(f64::from_be_bytes([
            u[i],
            u[i + 1],
            u[i + 2],
            u[i + 3],
            u[i + 4],
            u[i + 5],
            u[i + 6],
            u[i + 7],
        ]));
        i += 2;
    }

    return vec;
}

pub fn vec_uint8_to_float64_swapped(u: &Vec<u8>) -> Vec<f64> {
    let u_len = u.len();
    let v_range = u_len - u_len % 2;

    let mut vec = Vec::new();
    let mut i = 0;
    // Needs 8 elements in the range to create a f64 (+6 makes sure it's always 8 elements)
    while i + 6 < v_range {
        vec.push(f64::from_be_bytes([
            u[i + 2],
            u[i + 3],
            u[i],
            u[i + 1],
            u[i + 6],
            u[i + 7],
            u[i + 4],
            u[i + 5],
        ]));
        i += 2;
    }

    return vec;
}

#[cfg(test)]
mod tests {
    use crate::modbus::modbus_data_type_converters::float64::vec_uint8_to_float64;

    #[test]
    fn vec_uint8_converts_to_float64() {
        // https://en.wikipedia.org/wiki/Double-precision_floating-point_format
        assert_eq!(
            vec_uint8_to_float64(&vec![0, 0, 0, 0, 0, 0, 0, 0]),
            vec![0.0]
        ); // 0
        assert_eq!(
            vec_uint8_to_float64(&vec![128, 0, 0, 0, 0, 0, 0, 0]),
            vec![-0.0]
        ); // -0
        assert_eq!(
            vec_uint8_to_float64(&vec![192, 0, 0, 0, 0, 0, 0, 0]),
            vec![-2.0]
        ); // -2
        assert_eq!(
            vec_uint8_to_float64(&vec![63, 240, 0, 0, 0, 0, 0, 0]),
            vec![1.0]
        ); // 1
        assert_eq!(
            vec_uint8_to_float64(&vec![127, 240, 0, 0, 0, 0, 0, 0]),
            vec![f64::INFINITY]
        ); // Infinity
        assert_eq!(
            vec_uint8_to_float64(&vec![255, 240, 0, 0, 0, 0, 0, 0]),
            vec![f64::NEG_INFINITY]
        ); // -Infinity
        assert_eq!(
            vec_uint8_to_float64(&vec![64, 9, 33, 251, 84, 68, 45, 24]),
            vec![std::f64::consts::PI]
        ); // PI

        assert_eq!(
            vec_uint8_to_float64(&vec![0, 0, 0, 0, 0, 0, 0, 1]),
            vec![4.9406564584124654e-324]
        ); // Smallest positive subnormal number
        assert_eq!(
            vec_uint8_to_float64(&vec![0, 15, 255, 255, 255, 255, 255, 255]),
            vec![2.2250738585072009e-308]
        ); // Largest subnormal number
        assert_eq!(
            vec_uint8_to_float64(&vec![0, 16, 0, 0, 0, 0, 0, 0]),
            vec![2.2250738585072014e-308]
        ); // Smallest positive normal number
        assert_eq!(
            vec_uint8_to_float64(&vec![127, 239, 255, 255, 255, 255, 255, 255]),
            vec![1.7976931348623157e308]
        ); // Largest normal number
        assert_eq!(
            vec_uint8_to_float64(&vec![63, 240, 0, 0, 0, 0, 0, 1]),
            vec![1.0000000000000002]
        ); // Smallest number larger than one

        // Empty
        assert_eq!(vec_uint8_to_float64(&vec![]), vec![] as Vec<f64>);
        assert_eq!(vec_uint8_to_float64(&vec![100]), vec![] as Vec<f64>);
        assert_eq!(vec_uint8_to_float64(&vec![100, 20]), vec![] as Vec<f64>);
        assert_eq!(vec_uint8_to_float64(&vec![0, 45, 154]), vec![] as Vec<f64>);
        assert_eq!(vec_uint8_to_float64(&vec![0, 0, 0]), vec![] as Vec<f64>);
        assert_eq!(vec_uint8_to_float64(&vec![0, 0, 0, 12]), vec![] as Vec<f64>);
        assert_eq!(
            vec_uint8_to_float64(&vec![0, 0, 0, 12, 2]),
            vec![] as Vec<f64>
        );
        assert_eq!(
            vec_uint8_to_float64(&vec![0, 0, 0, 12, 2, 15, 1]),
            vec![] as Vec<f64>
        );
    }
}
