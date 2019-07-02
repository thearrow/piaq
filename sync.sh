# !/bin/sh
HOST="pi"
DIR="/home/pi/dev"

# sync dir to pi
rsync -avz --delete --exclude=.git --exclude=target --exclude=__pycache__ . $HOST:$DIR