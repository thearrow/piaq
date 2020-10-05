use std::{thread, time::Duration};

use bme680::PowerMode;
use regex::Regex;
use sds011::SDS011;

mod sensors {
    pub mod bme680;
}

fn main() {
    println!("\npiaq starting...\n");

    const READING_INTERVAL: Duration = Duration::from_secs(30);
    const SDS_WORK_INTERVAL_MINUTES: u8 = 5;

    let mut bme = sensors::bme680::init("/dev/i2c-1");
    let mut sds = SDS011::new("/dev/ttyUSB0").unwrap();
    sds.set_work_period(SDS_WORK_INTERVAL_MINUTES).unwrap();
    let pm25re = Regex::new(r"pm25: (.*),").unwrap();
    let pm10re = Regex::new(r"pm10: (.*) }").unwrap();

    loop {
        thread::sleep(READING_INTERVAL);

        bme.set_sensor_mode(PowerMode::ForcedMode).unwrap();
        let (data, _state) = bme.get_sensor_data().unwrap();
        println!("\nTemperature {}°C", data.temperature_celsius());
        println!("Pressure {}hPa", data.pressure_hpa());
        println!("Humidity {}%", data.humidity_percent());
        println!("Gas Resistence {}Ω", data.gas_resistance_ohm());

        // need to parse pm readings from string since library struct fields are private
        let dust = sds.query().unwrap();
        let duststr = format!("{:?}", dust);
        let pm25 = &pm25re.captures(&duststr).unwrap()[1];
        let pm10 = &pm10re.captures(&duststr).unwrap()[1];
        println!("PM2.5 {}", pm25);
        println!("PM10 {}", pm10);
    }
}
