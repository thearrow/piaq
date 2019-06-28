# !/bin/sh
HOST="pi@192.168.1.229"
DIR="/home/pi/dev"

# sync dir to pi
rsync -avz --exclude=.git --exclude=target . $HOST:$DIR