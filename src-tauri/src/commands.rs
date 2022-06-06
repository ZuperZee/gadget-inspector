use tokio_modbus::prelude::*;

use crate::modbus_data_type_converters::{
    float32::vec_uint8_to_float32, float64::vec_uint8_to_float64, sint16::vec_uint16_to_sint16,
    sint32::vec_uint32_to_sint32, sint64::vec_uint64_to_sint64, sint8::vec_uint8_to_sint8,
    uint32::vec_uint8_to_uint32, uint64::vec_uint8_to_uint64, uint8::vec_uint16_to_uint8,
};

#[derive(serde::Serialize)]
pub struct ModbusData {
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
}

#[tauri::command]
pub async fn read_modbus_command(
    socket_address: &str,
    address: u16,
    quantity: u16,
) -> Result<ModbusData, String> {
    let socket_addr = socket_address.parse().unwrap();

    let mut ctx = match tcp::connect(socket_addr).await {
        Ok(r) => r,
        Err(e) => {
            println!("{}", e);
            return Err(format!(
                "Failed connecting to socket address: {} with error: {:?}",
                socket_addr, e
            ));
        }
    };

    println!(
        "Reading input register address: {} quantity: {}",
        address, quantity
    );
    let res_uint16 = match ctx.read_input_registers(address, quantity).await {
        Ok(r) => r,
        Err(e) => {
            ctx.disconnect().await.ok(); // Try to disconnect before returning error
            println!("{}", e);
            return Err(format!(
                "Failed reading input address: {} quantity: {} with error: {:?}",
                address, quantity, e,
            ));
        }
    };

    let addresses: Vec<u16> = (address..address + quantity).collect();

    let res_uint8 = vec_uint16_to_uint8(&res_uint16);
    let res_uint32 = vec_uint8_to_uint32(&res_uint8);
    let res_uint64 = vec_uint8_to_uint64(&res_uint8);

    let res_sint8 = vec_uint8_to_sint8(&res_uint8);
    let res_sint16 = vec_uint16_to_sint16(&res_uint16);
    let res_sint32 = vec_uint32_to_sint32(&res_uint32);
    let res_sint64 = vec_uint64_to_sint64(&res_uint64);

    let res_float32 = vec_uint8_to_float32(&res_uint8);
    let res_float64 = vec_uint8_to_float64(&res_uint8);

    ctx.disconnect().await.unwrap(); // Disconnect after reading values

    return Ok(ModbusData {
        addresses: addresses,

        uint8: res_uint8,
        uint16: res_uint16,
        uint32: res_uint32,
        uint64: res_uint64,

        sint8: res_sint8,
        sint16: res_sint16,
        sint32: res_sint32,
        sint64: res_sint64,

        float32: res_float32,
        float64: res_float64,
    });
}

#[tauri::command]
pub async fn check_modbus_socket_address_command(socket_address: &str) -> Result<bool, String> {
    let socket_addr = socket_address.parse().unwrap();
    match tcp::connect(socket_addr).await {
        Ok(mut ctx) => {
            ctx.disconnect().await.ok(); // Try Disconnecting before returning
            return Ok(true);
        }
        Err(e) => {
            println!("{}", e);
            return Err(format!(
                "Failed connecting to socket address: {} with error: {:?}",
                socket_addr, e
            ));
        }
    };
}
