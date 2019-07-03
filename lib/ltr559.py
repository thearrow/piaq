import ltr559


def get_lux():
    try:
        lux = ltr559.get_lux()
    except IOError:
        lux = 0.0
    return lux

