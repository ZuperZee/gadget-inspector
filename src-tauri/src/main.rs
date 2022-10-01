#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod modbus_data_type_converters;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::read_modbus_address_command,
            commands::read_modbus_bit_address_command,
            commands::check_modbus_socket_address_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
