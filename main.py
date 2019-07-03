#!/usr/bin/env python3.7

import time
import sys
from timeloop import Timeloop
from datetime import timedelta

from lib.bme680 import get_env_sensor, get_env_data
from lib.ltr559 import get_lux
from lib.sds011 import get_pm_sensor, get_pm_data
from lib.logger import get_logger


def main():
    logger = get_logger()
    env_sensor = get_env_sensor()
    pm_sensor = get_pm_sensor()
    tl = Timeloop()

    @tl.job(interval=timedelta(seconds=5))
    def get_all_data():
        temp, hum, press = get_env_data(env_sensor)
        pm25, pm10 = get_pm_data(pm_sensor)
        lux = get_lux()
        logger.info(
            f"{temp:.1f}F, {hum:.1f}%RH, {press:.1f}inHg, {pm25:.1f}PM2.5, {pm10:.1f}PM10, {lux:.1f}lux"
        )

    tl.start()
    try:
        while True:
            time.sleep(5)
    except KeyboardInterrupt:
        logger.info("Shutting Down...")
        tl.stop()
    sys.exit(0)


if __name__ == "__main__":
    main()
