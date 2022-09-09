//
// echo.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Jul 05 2022
//

use anyhow::Context;
use clap::Parser;

use embedded_hal_one::can::{nb::Can, Frame, Id, StandardId};
use nb::block;
use socketcan_hal::{CanFrame, CanSocket};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// CAN interface
    #[clap(value_parser)]
    interface: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let can_interface = args.interface;

    let mut socket = CanSocket::open(&can_interface)
        .with_context(|| format!("Failed to open socket on interface {}", can_interface))?;
    socket
        .set_nonblocking(true)
        .with_context(|| "Failed to make socket non-blocking".to_string())?;

    let frame = block!(socket.receive());

    if let Ok(frame) = frame {
        println!("{}", frame_to_string(&frame));
    }

    let write_frame = CanFrame::new(StandardId::new(0x1f1).unwrap(), &[1, 2, 3, 4])
        .expect("Failed to create CAN frame");

    if let Err(e) = block!(socket.transmit(&write_frame)) {
        println!("{}", e);
    }

    Ok(())
}

fn frame_to_string<F: Frame>(f: &F) -> String {
    let id = get_raw_id(&f.id());
    let data_string = f
        .data()
        .iter()
        .fold(String::from(""), |a, b| format!("{} {:02x}", a, b));

    format!("{:08X}  [{}] {}", id, f.dlc(), data_string)
}

fn get_raw_id(id: &Id) -> u32 {
    match id {
        Id::Standard(id) => id.as_raw() as u32,
        Id::Extended(id) => id.as_raw(),
    }
}
