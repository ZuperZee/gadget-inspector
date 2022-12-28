use std::{net::ToSocketAddrs, time::Duration};

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

    let mut ctx = match timeout(
        TIMEOUT_DURATION,
        tcp::connect_slave(socket_addr, Slave(slave_id)),
    )
    .await
    {
        Ok(o) => match o {
            Ok(r) => r,
            Err(e) => {
                return Err(format!(
                    "Failed connecting to socket address: {} with error: {:?}",
                    socket_addr, e
                ));
            }
        },
        Err(_) => {
            return Err(format!(
                "Timed out connecting to socket address: {} with slave id: {}",
                socket_addr, slave_id,
            ))
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
            Ok(true)
        }
        Err(e) => Err(format!(
            "Failed connecting to socket address: {} with error: {:?}",
            socket_addr, e
        )),
    }
}
