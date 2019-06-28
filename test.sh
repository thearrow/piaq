# !/bin/sh
HOST="pi"
DIR="/home/pi/dev"

# sync dir to pi
rsync -avz --exclude=.git --exclude=target . $HOST:$DIR

# ssh to pi, compile, and run
ssh -q $HOST << ENDSSH
cd "$DIR"
cargo run
ENDSSH