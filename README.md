# PI IAQ

One-time setup:

- install rust
- `sudo apt-get update`
- `sudo apt-get install -y pkg-config libudev-dev` # (serialport-rs deps)
- `sudo apt-get install -y python3.7 python3-pip`
- `sudo python3.7 -m pip install --upgrade pip`
- `python3.7 -m pip install --user -r requirements.txt`

Development/testing:

- ./test.sh
