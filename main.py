#!/usr/bin/env python3.7

import bme680
from time import sleep
from lib.temp import get_compensated_temp

try:
    sensor = bme680.BME680(bme680.I2C_ADDR_PRIMARY)
except IOError:
    sensor = bme680.BME680(bme680.I2C_ADDR_SECONDARY)

sensor.set_humidity_oversample(bme680.OS_8X)
sensor.set_pressure_oversample(bme680.OS_8X)
sensor.set_temperature_oversample(bme680.OS_8X)
sensor.set_filter(bme680.FILTER_SIZE_3)

try:
    while True:
        if sensor.get_sensor_data():
            comp_temp = get_compensated_temp(sensor.data.temperature)
            output = "{0:.2f}C, {1:.3f}%RH, {2:.2f}hPa".format(
                comp_temp, sensor.data.humidity, sensor.data.pressure
            )
            print(output)
        sleep(1)

except KeyboardInterrupt:
    pass
