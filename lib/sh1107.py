from luma.core.interface.serial import i2c
from luma.core.render import canvas
from luma.oled.device import sh1106


def get_oled():
    serial = i2c(port=1, address=0x3C)
    device = sh1106(serial, rotate=2, width=128, height=128)
    return device


def oled_draw(device, **kwargs):
    with canvas(device) as draw:
        draw.rectangle(device.bounding_box, fill="black")
        offset = 0
        for key, value in kwargs.items():
            draw.text((0, offset), f"{key}: {value:.1f}", fill="white")
            offset += 10


def oled_shutdown(device):
    device.cleanup()
