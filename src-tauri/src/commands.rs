use std::net::ToSocketAddrs;

use tokio_modbus::{
    prelude::{tcp, Reader},
    slave::Slave,
};

use crate::modbus::convert::{create_modbus_bit_data, create_modbus_numerical_data, ModbusData};

// Reads holding and input registers
#[tauri::command]
pub async fn read_modbus_address_command(
    socket_address: &str,
    slave_id: u8,
    address: u16,
    quantity: u16,
    function_code: u8,
) -> Result<ModbusData, String> {
    let mut socket_addr_iter = match socket_address.to_socket_addrs() {
        Ok(r) => r,
        Err(e) => {
            return Err(format!(
                "Failed parsing socket address: {} with error: {:?}",
                socket_address, e
            ));
        }
    };

    let socket_addr = match socket_addr_iter.next() {
        Some(r) => r,
        None => {
            return Err(format!("Couldn't find socket address: {}", socket_address));
        }
    };

    let mut ctx = match tcp::connect_slave(socket_addr, Slave(slave_id)).await {
        Ok(r) => r,
        Err(e) => {
            return Err(format!(
                "Failed connecting to socket address: {} with error: {:?}",
                socket_addr, e
            ));
        }
    };

    let addresses: Vec<u16> = (address..address + quantity).collect();

    if function_code == 1 {
        match ctx.read_coils(address, quantity).await {
            Ok(r) => {
                ctx.disconnect().await.ok();
                return Ok(create_modbus_bit_data(r, addresses));
            }
            Err(e) => {
                ctx.disconnect().await.ok(); // Try to disconnect before returning error
                return Err(format!(
                    "Failed reading coil address: {} quantity: {} with error: {:?}",
                    address, quantity, e,
                ));
            }
        }
    }

    if function_code == 2 {
        match ctx.read_discrete_inputs(address, quantity).await {
            Ok(r) => {
                ctx.disconnect().await.ok();
                return Ok(create_modbus_bit_data(r, addresses));
            }
            Err(e) => {
                ctx.disconnect().await.ok(); // Try to disconnect before returning error
                return Err(format!(
                    "Failed reading discrete input address: {} quantity: {} with error: {:?}",
                    address, quantity, e,
                ));
            }
        }
    }

    if function_code == 3 {
        match ctx.read_holding_registers(address, quantity).await {
            Ok(r) => {
                ctx.disconnect().await.ok();
                return Ok(create_modbus_numerical_data(r, addresses));
            }
            Err(e) => {
                ctx.disconnect().await.ok(); // Try to disconnect before returning error
                return Err(format!(
                    "Failed reading holding address: {} quantity: {} with error: {:?}",
                    address, quantity, e,
                ));
            }
        }
    }

    if function_code == 4 {
        match ctx.read_input_registers(address, quantity).await {
            Ok(r) => {
                ctx.disconnect().await.ok();
                return Ok(create_modbus_numerical_data(r, addresses));
            }
            Err(e) => {
                ctx.disconnect().await.ok(); // Try to disconnect before returning error
                return Err(format!(
                    "Failed reading input address: {} quantity: {} with error: {:?}",
                    address, quantity, e,
                ));
            }
        }
    }

    return Err(format!("Invalid function code: {}", function_code));
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
            return Err(format!(
                "Failed connecting to socket address: {} with error: {:?}",
                socket_addr, e
            ));
        }
    };
}
