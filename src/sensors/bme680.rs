extern crate bme680;
extern crate embedded_hal;
extern crate linux_embedded_hal as hal;

use bme680::*;
use hal::*;
use std::time::Duration;

pub fn init(port: &str) -> Bme680<I2cdev, Delay> {
    let i2c = I2cdev::new(port).unwrap();
    let mut dev = Bme680::init(i2c, Delay {}, I2CAddress::Primary).unwrap();

    let settings = SettingsBuilder::new()
        .with_humidity_oversampling(OversamplingSetting::OS2x)
        .with_pressure_oversampling(OversamplingSetting::OS4x)
        .with_temperature_oversampling(OversamplingSetting::OS8x)
        .with_temperature_filter(IIRFilterSize::Size3)
        .with_temperature_offset(-5.0)
        .with_gas_measurement(Duration::from_millis(2000), 320, 25)
        .with_run_gas(true)
        .build();

    dev.set_sensor_settings(settings).unwrap();
    // burn one reading for outliers
    dev.set_sensor_mode(PowerMode::ForcedMode).unwrap();
    let (_data, _state) = dev.get_sensor_data().unwrap();

    dev
}
