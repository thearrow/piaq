#!/usr/bin/env python

# https://github.com/pimoroni/bme680-python

import bme680
from time import sleep
from subprocess import PIPE, Popen

try:
    sensor = bme680.BME680(bme680.I2C_ADDR_PRIMARY)
except IOError:
    sensor = bme680.BME680(bme680.I2C_ADDR_SECONDARY)

sensor.set_humidity_oversample(bme680.OS_8X)
sensor.set_pressure_oversample(bme680.OS_8X)
sensor.set_temperature_oversample(bme680.OS_8X)
sensor.set_filter(bme680.FILTER_SIZE_3)


def get_cpu_temperature():
    process = Popen(["vcgencmd", "measure_temp"], stdout=PIPE, text=True)
    output, _error = process.communicate()
    return float(output[output.index("=") + 1 : output.rindex("'")])


factor = 10.0  # Smaller numbers adjust temp down, vice versa
smooth_size = 10  # Dampens jitter due to rapid CPU temp changes
cpu_temps = []

try:
    while True:
        if sensor.get_sensor_data():
            cpu_temp = get_cpu_temperature()
            cpu_temps.append(cpu_temp)
            if len(cpu_temps) > smooth_size:
                cpu_temps = cpu_temps[1:]

            smoothed_cpu_temp = sum(cpu_temps) / float(len(cpu_temps))
            raw_temp = sensor.data.temperature
            comp_temp = raw_temp - ((smoothed_cpu_temp - raw_temp) / factor)

            output = "{0:.2f}C, {1:.3f}%RH, {2:.2f}hPa".format(
                comp_temp, sensor.data.humidity, sensor.data.pressure
            )
            print(output)
        sleep(1)

except KeyboardInterrupt:
    pass
