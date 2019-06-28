# PI IAQ

One-time setup:

- install rust
- `sudo apt-get update`
- `sudo apt-get install -y pkg-config libudev-dev` # (serialport-rs deps)
- `sudo apt-get install -y python3.7 python3-pip`
- `python3.7 -m pip install pip`
- `python3.7 -m pip install -r requirements.txt`

Development/testing:

- ./test.sh
