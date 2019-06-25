extern crate serialport;

use serialport::prelude::*;
use serialport::SerialPortType;

use std::io::{self};
use std::time::Duration;

fn main() {
    println!("Hello, PIAQ!");

    if let Ok(ports) = serialport::available_ports() {
        match ports.len() {
            0 => println!("No ports found."),
            1 => println!("Found 1 port:"),
            n => println!("Found {} ports:", n),
        };
        for p in ports {
            println!("  {}", p.port_name);
            match p.port_type {
                SerialPortType::UsbPort(info) => {
                    println!("    Type: USB");
                    println!("    VID:{:04x} PID:{:04x}", info.vid, info.pid);
                    println!(
                        "     Serial Number: {}",
                        info.serial_number.as_ref().map_or("", String::as_str)
                    );
                    println!(
                        "      Manufacturer: {}",
                        info.manufacturer.as_ref().map_or("", String::as_str)
                    );
                    println!(
                        "           Product: {}",
                        info.product.as_ref().map_or("", String::as_str)
                    );
                }
                SerialPortType::BluetoothPort => {
                    println!("    Type: Bluetooth");
                }
                SerialPortType::PciPort => {
                    println!("    Type: PCI");
                }
                SerialPortType::Unknown => {
                    println!("    Type: Unknown");
                }
            }
        }
    } else {
        print!("Error listing serial ports");
    }

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
                    },
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