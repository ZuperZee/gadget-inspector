#[tokio::main(flavor = "current_thread")]
async fn read_modbus() -> Result<(), Box<dyn std::error::Error>> {
    use tokio_modbus::prelude::*;

    let socket_addr = "127.0.0.1:5502".parse().unwrap();

    let mut ctx = tcp::connect(socket_addr).await?;

    println!("Fetching the coupler ID");
    let res = ctx.read_input_registers(0x00, 7).await?;

    println!("The result is '{:?}'", res);

    Ok(())
}
