use std::{env, time::Duration};

use bme680::PowerMode;
use regex::Regex;
use sds011::SDS011;
use sqlx::postgres::PgPoolOptions;
use sqlx::Executor;
use tokio::time::sleep;

mod sensors {
    pub mod bme680;
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    println!("\npiaq starting...\n");

    const READING_INTERVAL: Duration = Duration::from_secs(60);
    const SDS_WORK_INTERVAL_MINUTES: u8 = 5;

    let (mut bme, mut delay) = sensors::bme680::init("/dev/i2c-1");
    let mut sds = SDS011::new("/dev/ttyUSB0").unwrap();
    sds.set_work_period(SDS_WORK_INTERVAL_MINUTES).unwrap();
    let pm25re = Regex::new(r"pm25: (.*),").unwrap();
    let pm10re = Regex::new(r"pm10: (.*) }").unwrap();

    let pg_uri: String = env::var("POSTGRES_URI").expect("POSTGRES_URI env var was not set!");
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&pg_uri)
        .await?;

    println!("creating db table if not exists...");
    pool.execute(sqlx::query(
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
    ))
    .await?;

    loop {
        sleep(READING_INTERVAL).await;

        bme.set_sensor_mode(&mut delay, PowerMode::ForcedMode)
            .unwrap();
        let (data, _state) = bme.get_sensor_data(&mut delay).unwrap();

        // need to parse pm readings from string since library struct fields are private
        let dust = sds.query().unwrap();
        let dust_str = format!("{:?}", dust);
        let pm25 = &pm25re.captures(&dust_str).unwrap()[1];
        let pm10 = &pm10re.captures(&dust_str).unwrap()[1];

        pool.execute(
            sqlx::query(
                "
                INSERT INTO piaq 
                (
                    temperature, 
                    humidity,
                    pressure,
                    gas,
                    pm25,
                    pm10
                )
                VALUES ($1, $2, $3, $4, $5, $6)
                ",
            )
            .bind(&data.temperature_celsius())
            .bind(&data.humidity_percent())
            .bind(&data.pressure_hpa())
            .bind(&(data.gas_resistance_ohm() as f32))
            .bind(&pm25.parse::<f32>().unwrap())
            .bind(&pm10.parse::<f32>().unwrap()),
        )
        .await?;
    }
}
