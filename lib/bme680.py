import bme680
from subprocess import PIPE, Popen

factor = 8.0  # Smaller numbers adjust temp down, vice versa
smooth_size = 10  # Dampens jitter due to rapid CPU temp changes
cpu_temps = []


def get_cpu_temperature():
    process = Popen(["vcgencmd", "measure_temp"], stdout=PIPE, text=True)
    output, _error = process.communicate()
    return float(output[output.index("=") + 1 : output.rindex("'")])


def c_to_f(c):
    return c * (9.0 / 5.0) + 32.0


def get_compensated_temp(raw_temp):
    """
    Get the ambient temperature compensated for the current temp of the raspberry pi CPU
    
    Arguments:
        raw_temp {float} -- raw (uncompensated) temperature in C
    
    Returns:
        float -- compensated temperature in F
    """
    global cpu_temps
    cpu_temp = get_cpu_temperature()
    cpu_temps.append(cpu_temp)
    if len(cpu_temps) > smooth_size:
        cpu_temps = cpu_temps[1:]
    smoothed_cpu_temp = sum(cpu_temps) / float(len(cpu_temps))
    comp_temp = raw_temp - ((smoothed_cpu_temp - raw_temp) / factor)
    return c_to_f(comp_temp)


def get_env_sensor():
    """
    Get the bme680 sensor instance
    """
    try:
        env_sensor = bme680.BME680(bme680.I2C_ADDR_PRIMARY)
    except IOError:
        env_sensor = bme680.BME680(bme680.I2C_ADDR_SECONDARY)
    env_sensor.set_humidity_oversample(bme680.OS_8X)
    env_sensor.set_pressure_oversample(bme680.OS_8X)
    env_sensor.set_temperature_oversample(bme680.OS_8X)
    env_sensor.set_filter(bme680.FILTER_SIZE_3)
    return env_sensor


def get_env_data(env_sensor):
    """
    Get data (temp, humidity, pressure) from the bme680 sensor
    
    Arguments:
        env_sensor (bme680 sensor)
    
    Returns:
        temp (F), humidity (%), pressure (inHg)
    """
    if env_sensor.get_sensor_data():
        temp = get_compensated_temp(env_sensor.data.temperature)
        hum = env_sensor.data.humidity
        press = env_sensor.data.pressure / 33.863886666667
        return temp, hum, press
    return None, None, None

