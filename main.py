#!/usr/bin/env python3.7

import bme680
import time
import sys
from timeloop import Timeloop
from datetime import timedelta

from lib.temp import get_compensated_temp
from lib.sds011 import SDS011
from lib.custom_log import setup_logger


def main():
    logger = setup_logger()
    try:
        env_sensor = bme680.BME680(bme680.I2C_ADDR_PRIMARY)
    except IOError:
        env_sensor = bme680.BME680(bme680.I2C_ADDR_SECONDARY)

    try:
        pm_sensor = SDS011("/dev/ttyUSB0", use_query_mode=True)
    except IOError:
        pm_sensor = None
        logger.warning("Could not connect to SDS011 device!")

    env_sensor.set_humidity_oversample(bme680.OS_8X)
    env_sensor.set_pressure_oversample(bme680.OS_8X)
    env_sensor.set_temperature_oversample(bme680.OS_8X)
    env_sensor.set_filter(bme680.FILTER_SIZE_3)

    tl = Timeloop()

    @tl.job(interval=timedelta(seconds=5))
    def get_env_data():
        if env_sensor.get_sensor_data():
            temp = get_compensated_temp(env_sensor.data.temperature)
            hum = env_sensor.data.humidity
            press = env_sensor.data.pressure / 33.863886666667
            logger.info(f"{temp:.1f}F, {hum:.1f}%RH, {press:.1f}inHg")

    @tl.job(interval=timedelta(seconds=5))
    def get_pm_data():
        if pm_sensor:
            pm_data = pm_sensor.query()
        else:
            pm_data = (0, 0)
        logger.info(f"{pm_data[0]:.1f}PM2.5, {pm_data[1]:.1f}PM10")

    tl.start()
    try:
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        logger.info("Shutting Down...")
        tl.stop()
    sys.exit(0)


if __name__ == "__main__":
    main()
