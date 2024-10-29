use std::time::Duration;

use itertools::Itertools;
use tokio::time::sleep;
use tokio_modbus::{
    client::{rtu, Reader, Writer},
    Slave,
};
use tokio_serial::SerialStream;

#[tokio::main]
async fn main() {
    let builder = tokio_serial::new("/dev/ttyACM0", 9600);
    let stream = SerialStream::open(&builder).unwrap();
    let mut ctx = rtu::attach_slave(stream, Slave(1));

    let rsp = ctx
        .read_holding_registers(0x4000, 1)
        .await
        .unwrap()
        .unwrap();
    println!("Device address: {}", rsp[0]);
    let rsp = ctx
        .read_holding_registers(0x8000, 1)
        .await
        .unwrap()
        .unwrap();
    let version = (rsp[0] / 100, rsp[0] % 100);
    println!("Version: {}.{}", version.0, version.1);

    //let rsp = ctx.read_coils(0x0000, 0x0010).await.unwrap();

    loop {
        let mut flags = [false; 16];
        for x in 0..16 {
            flags[x] = true;
            // MSB->LSB
            let lsb = [
                flags[8], flags[9], flags[10], flags[11], flags[12], flags[13], flags[14],
                flags[15], flags[0], flags[1], flags[2], flags[3], flags[4], flags[5], flags[6],
                flags[7],
            ];
            ctx.write_multiple_coils(0x0000, &lsb).await.unwrap();
            println!(
                "Relais: {} | {}",
                flags[0..8]
                    .iter()
                    .map(|&r| if r { "1" } else { "0" })
                    .join(" "),
                flags[8..16]
                    .iter()
                    .map(|&r| if r { "1" } else { "0" })
                    .join(" ")
            );
            sleep(Duration::from_millis(500)).await;
            flags[x] = false;
        }
    }
}
