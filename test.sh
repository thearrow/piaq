# !/bin/sh
HOST="pi"
DIR="/home/pi/dev"

# sync dir to pi
rsync -avz --delete --exclude=.git --exclude=target --exclude=__pycache__ . $HOST:$DIR

# ssh to pi, compile, and run
ssh -tt $HOST << ENDSSH
cd $DIR
python3.7 -u ./main.py
ENDSSH