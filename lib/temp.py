from subprocess import PIPE, Popen

factor = 10.0  # Smaller numbers adjust temp down, vice versa
smooth_size = 10  # Dampens jitter due to rapid CPU temp changes
cpu_temps = []


def get_cpu_temperature():
    process = Popen(["vcgencmd", "measure_temp"], stdout=PIPE, text=True)
    output, _error = process.communicate()
    return float(output[output.index("=") + 1 : output.rindex("'")])


def get_compensated_temp(raw_temp):
    global cpu_temps
    cpu_temp = get_cpu_temperature()
    cpu_temps.append(cpu_temp)
    if len(cpu_temps) > smooth_size:
        cpu_temps = cpu_temps[1:]
    smoothed_cpu_temp = sum(cpu_temps) / float(len(cpu_temps))
    comp_temp = raw_temp - ((smoothed_cpu_temp - raw_temp) / factor)
    return comp_temp
