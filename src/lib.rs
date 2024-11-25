// Chongyou Car Control
// Author：Leon <echo_ai@foxmail.com>

mod command;
mod enums;
mod error;

use std::{io::Write, sync::mpsc::Sender, time::Duration};

use error::Error;
use serial::SerialPort;
use std::sync::mpsc;

/// 连接小车并控制
///
/// ```rust
/// let listen = chongyoucar::connect("/dev/ttyUSB0").unwrap();
///
/// listen.send((1.0, 0.0)).ok();
/// std::thread::sleep(std::time::Duration::from_secs(2));
/// listen.send((0.0, 1.0)).ok();
/// ```
pub fn connect(serial_port: &str) -> anyhow::Result<Sender<(f64, f64)>> {
    // connect serial
    const COM_SETTINGS: serial::PortSettings = serial::PortSettings {
        baud_rate: serial::Baud115200,
        char_size: serial::Bits8,
        parity: serial::ParityNone,
        stop_bits: serial::Stop1,
        flow_control: serial::FlowNone,
    };

    let mut com = serial::open(&serial_port).or(Err(Error::Connect))?;
    com.configure(&COM_SETTINGS).or(Err(Error::SettingsSet))?;
    com.set_timeout(Duration::from_millis(1000))
        .or(Err(Error::SetTimeout))?;

    // msg channel
    let (tx_key, rx_key) = mpsc::channel::<(f64, f64)>();

    std::thread::spawn(move || {
        while let Ok((x, w)) = rx_key.recv() {
            let data = command::send_speed_to_x4chassis(x, 0.0, w);
            com.write_all(&data).ok();
        }
    });

    Ok(tx_key)
}
