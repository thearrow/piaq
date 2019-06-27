extern crate serialport;

use serialport::prelude::*;
use std::io;
use std::time::Duration;

mod list_ports;

fn main() {
    list_ports::list_ports();

    let s = SerialPortSettings {
        baud_rate: 9600,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::from_millis(1),
    };

    let device = "/dev/ttyUSB0";

    match serialport::open_with_settings(device, &s) {
        Ok(mut port) => {
            let mut buffer: Vec<u8> = vec![0; 10];
            println!("Receiving data on {}:", &device);
            loop {
                match port.read(buffer.as_mut_slice()) {
                    Ok(_t) => {
                        let pm25 = (buffer[3] as f32 * 256. + buffer[2] as f32) / 10.0;
                        let pm10 = (buffer[5] as f32 * 256. + buffer[4] as f32) / 10.0;
                        println!("data = pm2.5: {}, pm10: {}", pm25, pm10);
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", device, e);
            ::std::process::exit(1);
        }
    }
}