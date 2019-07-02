# !/bin/sh
HOST="pi"
DIR="/home/pi/dev"

# sync dir to pi
rsync -avz --delete --exclude=.git --exclude=target . $HOST:$DIR

# ssh to pi, compile, and run
ssh -qT $HOST << ENDSSH
cd $DIR
python3.7 -u ./main.py
ENDSSH