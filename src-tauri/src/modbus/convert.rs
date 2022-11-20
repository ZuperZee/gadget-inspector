use super::modbus_data_type_converters::{
    ascii::vec_uint8_to_ascii,
    float32::{vec_uint8_to_float32, vec_uint8_to_float32_swapped},
    float64::{vec_uint8_to_float64, vec_uint8_to_float64_swapped},
    sint16::vec_uint16_to_sint16,
    sint32::vec_uint32_to_sint32,
    sint64::vec_uint64_to_sint64,
    sint8::vec_uint8_to_sint8,
    uint32::{vec_uint8_to_uint32, vec_uint8_to_uint32_swapped},
    uint64::{vec_uint8_to_uint64, vec_uint8_to_uint64_swapped},
    uint8::vec_uint16_to_uint8,
};

#[derive(serde::Serialize)]
pub enum ModbusData {
    ModbusNumericalData {
        addresses: Vec<u16>,

        uint8: Vec<u8>,
        uint16: Vec<u16>,
        uint32: Vec<u32>,
        uint64: Vec<u64>,

        sint8: Vec<i8>,
        sint16: Vec<i16>,
        sint32: Vec<i32>,
        sint64: Vec<i64>,

        float32: Vec<f32>,
        float64: Vec<f64>,

        ascii: Vec<char>,
    },
    ModbusBitData {
        addresses: Vec<u16>,
        bool: Vec<bool>,
    },
}

pub fn create_modbus_numerical_data(
    raw_uint16: Vec<u16>,
    addresses: Vec<u16>,
    is_byte_swap: bool,
    is_word_swap: bool,
) -> ModbusData {
    let uint16 = match is_byte_swap {
        true => raw_uint16.iter().map(|r| r.swap_bytes()).collect(),
        false => raw_uint16,
    };

    let uint8 = vec_uint16_to_uint8(&uint16);
    let sint8 = vec_uint8_to_sint8(&uint8);
    let sint16 = vec_uint16_to_sint16(&uint16);

    // Word swap only does anything for 32 and 64 bits
    let (uint32, uint64, float32, float64) = match is_word_swap {
        true => (
            vec_uint8_to_uint32_swapped(&uint8),
            vec_uint8_to_uint64_swapped(&uint8),
            vec_uint8_to_float32_swapped(&uint8),
            vec_uint8_to_float64_swapped(&uint8),
        ),
        false => (
            vec_uint8_to_uint32(&uint8),
            vec_uint8_to_uint64(&uint8),
            vec_uint8_to_float32(&uint8),
            vec_uint8_to_float64(&uint8),
        ),
    };

    // Sint uses their equivalent uint variable
    let sint32 = vec_uint32_to_sint32(&uint32);
    let sint64 = vec_uint64_to_sint64(&uint64);

    let ascii = vec_uint8_to_ascii(&uint8);

    return ModbusData::ModbusNumericalData {
        addresses,

        uint8,
        uint16,
        uint32,
        uint64,

        sint8,
        sint16,
        sint32,
        sint64,

        float32,
        float64,

        ascii,
    };
}

pub fn create_modbus_bit_data(bool: Vec<bool>, addresses: Vec<u16>) -> ModbusData {
    return ModbusData::ModbusBitData { addresses, bool };
}
