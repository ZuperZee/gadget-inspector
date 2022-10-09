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

    let address_to_u32 = u32::from(address) + u32::from(quantity);
    if address_to_u32 > u16::MAX.into() {
        return Err(format!(
            "Max address is {}, but got {} (address + quantity)",
            u16::MAX,
            address_to_u32
        ));
    }

    let address_to: u16 = address_to_u32.try_into().unwrap_or(u16::MAX); // Read to max address if it fails to cast

    let addresses: Vec<u16> = (address..address_to).collect();

    let res = match function_code {
        1 => ctx
            .read_coils(address, quantity)
            .await
            .map(|r| create_modbus_bit_data(r, addresses)),
        2 => ctx
            .read_discrete_inputs(address, quantity)
            .await
            .map(|r| create_modbus_bit_data(r, addresses)),
        3 => ctx
            .read_holding_registers(address, quantity)
            .await
            .map(|r| create_modbus_numerical_data(r, addresses)),
        4 => ctx
            .read_input_registers(address, quantity)
            .await
            .map(|r| create_modbus_numerical_data(r, addresses)),
        _ => {
            ctx.disconnect().await.ok(); // Try to disconnect before returning error
            return Err(format!("Invalid function code: {}", function_code));
        }
    };

    ctx.disconnect().await.ok(); // Disconnect after reading

    match res {
        Ok(r) => Ok(r),
        Err(e) => Err(format!(
            "Failed reading modbus address: {} quantity: {} with error: {:?}",
            address, quantity, e,
        )),
    }
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
