use std::{net::SocketAddr, time::Duration};

use tokio::time::timeout;
use tokio_modbus::{
    prelude::{tcp, Reader},
    slave::Slave,
};

use crate::modbus::convert::{create_modbus_bit_data, create_modbus_numerical_data, ModbusData};

const TIMEOUT_DURATION: Duration = Duration::from_millis(500);

// Reads holding and input registers
#[tauri::command]
pub async fn read_modbus_address_command(
    socket_address: &str,
    slave_id: u8,
    address: u16,
    quantity: u16,
    function_code: u8,
    is_byte_swap: bool,
    is_word_swap: bool,
) -> Result<ModbusData, String> {
    let socket_addr: SocketAddr = socket_address.parse().map_err(|err| {
        format!(
            "Couldn't parse socket address: {} with error: {}",
            socket_address, err
        )
    })?;

    let mut ctx = timeout(
        TIMEOUT_DURATION,
        tcp::connect_slave(socket_addr, Slave(slave_id)),
    )
    .await
    .map_err(|_| {
        format!(
            "Timed out connecting to socket address: {} with slave id: {}",
            socket_addr, slave_id,
        )
    })?
    .map_err(|err| {
        format!(
            "Failed connecting to socket address: {} with error: {:?}",
            socket_addr, err
        )
    })?;

    let address_to = address.checked_add(quantity).ok_or_else(|| {
        format!(
            "Address {} + quantity {} is more than max address {}",
            address,
            quantity,
            u16::MAX,
        )
    })?;
    let addresses: Vec<u16> = (address..address_to).collect();

    let res = match function_code {
        1 => match timeout(TIMEOUT_DURATION, ctx.read_coils(address, quantity)).await {
            Ok(o) => o.map(|r| create_modbus_bit_data(r, addresses)),
            Err(_) => {
                return Err(format!(
                    "Timed out reading modbus address: {} quantity: {}",
                    address, quantity,
                ))
            }
        },
        2 => match timeout(
            TIMEOUT_DURATION,
            ctx.read_discrete_inputs(address, quantity),
        )
        .await
        {
            Ok(o) => o.map(|r| create_modbus_bit_data(r, addresses)),
            Err(_) => {
                return Err(format!(
                    "Timed out reading modbus address: {} quantity: {}",
                    address, quantity,
                ))
            }
        },
        3 => match timeout(
            TIMEOUT_DURATION,
            ctx.read_holding_registers(address, quantity),
        )
        .await
        {
            Ok(o) => {
                o.map(|r| create_modbus_numerical_data(r, addresses, is_byte_swap, is_word_swap))
            }
            Err(_) => {
                return Err(format!(
                    "Timed out reading modbus address: {} quantity: {}",
                    address, quantity,
                ))
            }
        },
        4 => match timeout(
            TIMEOUT_DURATION,
            ctx.read_input_registers(address, quantity),
        )
        .await
        {
            Ok(o) => {
                o.map(|r| create_modbus_numerical_data(r, addresses, is_byte_swap, is_word_swap))
            }
            Err(_) => {
                return Err(format!(
                    "Timed out reading modbus address: {} quantity: {}",
                    address, quantity,
                ))
            }
        },
        _ => {
            timeout(TIMEOUT_DURATION, ctx.disconnect()).await.ok(); // Try to disconnect before returning error
            return Err(format!("Invalid function code: {}", function_code));
        }
    };

    timeout(TIMEOUT_DURATION, ctx.disconnect()).await.ok();

    res.map_err(|e| {
        format!(
            "Failed reading modbus address: {} quantity: {} with error: {:?}",
            address, quantity, e,
        )
    })
}

#[tauri::command]
pub async fn check_modbus_socket_address_command(socket_address: &str) -> Result<bool, String> {
    let socket_addr = socket_address.parse().unwrap();
    match tcp::connect(socket_addr).await {
        Ok(mut ctx) => {
            ctx.disconnect().await.ok(); // Try Disconnecting before returning
            Ok(true)
        }
        Err(e) => Err(format!(
            "Failed connecting to socket address: {} with error: {:?}",
            socket_addr, e
        )),
    }
}
