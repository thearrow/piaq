use std::{env, thread, time::Duration};

use bme680::PowerMode;
use postgres::{Client, NoTls};
use regex::Regex;
use sds011::SDS011;

mod sensors {
    pub mod bme680;
}

fn main() {
    println!("\npiaq starting...\n");

    const READING_INTERVAL: Duration = Duration::from_secs(60);
    const SDS_WORK_INTERVAL_MINUTES: u8 = 5;
    let pg_host: String = env::var("POSTGRES_HOST").unwrap();
    let pg_dbname: String = env::var("POSTGRES_DB").unwrap();
    let pg_user: String = env::var("POSTGRES_USER").unwrap();
    let pg_pass: String = env::var("POSTGRES_PASSWORD").unwrap();

    let mut bme = sensors::bme680::init("/dev/i2c-1");
    let mut sds = SDS011::new("/dev/ttyUSB0").unwrap();
    sds.set_work_period(SDS_WORK_INTERVAL_MINUTES).unwrap();
    let pm25re = Regex::new(r"pm25: (.*),").unwrap();
    let pm10re = Regex::new(r"pm10: (.*) }").unwrap();

    let mut client = Client::configure()
        .host(&pg_host)
        .dbname(&pg_dbname)
        .user(&pg_user)
        .password(&pg_pass)
        .connect(NoTls)
        .unwrap();

    client
        .batch_execute(
            "
        CREATE TABLE IF NOT EXISTS piaq (
            time          TIMESTAMPTZ NOT NULL PRIMARY KEY DEFAULT CURRENT_TIMESTAMP,
            temperature   REAL NULL,
            humidity      REAL NULL,
            pressure      REAL NULL,
            gas           REAL NULL,
            pm25          REAL NULL,
            pm10          REAL NULL
        )
    ",
        )
        .unwrap();

    loop {
        thread::sleep(READING_INTERVAL);

        bme.set_sensor_mode(PowerMode::ForcedMode).unwrap();
        let (data, _state) = bme.get_sensor_data().unwrap();

        // need to parse pm readings from string since library struct fields are private
        let dust = sds.query().unwrap();
        let duststr = format!("{:?}", dust);
        let pm25 = &pm25re.captures(&duststr).unwrap()[1];
        let pm10 = &pm10re.captures(&duststr).unwrap()[1];

        client
            .execute(
                "INSERT INTO piaq (
                temperature, 
                humidity,
                pressure,
                gas,
                pm25,
                pm10
            )
            VALUES ($1, $2, $3, $4, $5, $6)",
                &[
                    &data.temperature_celsius(),
                    &data.humidity_percent(),
                    &data.pressure_hpa(),
                    &(data.gas_resistance_ohm() as f32),
                    &pm25.parse::<f32>().unwrap(),
                    &pm10.parse::<f32>().unwrap(),
                ],
            )
            .unwrap();
    }
}
