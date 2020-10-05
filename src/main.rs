use std::{thread, time};

use bme680::PowerMode;

mod sensors {
    pub mod bme680;
}

fn main() {
    println!("\npiaq starting...\n");

    let mut bme = sensors::bme680::init();

    loop {
        thread::sleep(time::Duration::from_secs(10));
        bme.set_sensor_mode(PowerMode::ForcedMode).unwrap();
        let (data, _state) = bme.get_sensor_data().unwrap();
        println!("\nTemperature {}°C", data.temperature_celsius());
        println!("Pressure {}hPa", data.pressure_hpa());
        println!("Humidity {}%", data.humidity_percent());
        println!("Gas Resistence {}Ω", data.gas_resistance_ohm());
    }
}
