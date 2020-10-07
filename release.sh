#!/bin/bash

cross build --release --target armv7-unknown-linux-musleabihf

if [ $? -eq 0 ]
then
    echo "compilation succeeded, uploading..."
else
    echo "compilation failed! exiting..."
    exit 1
fi

rsync -azvhP target/armv7-unknown-linux-musleabihf/release/piaq pi:/home/ubuntu/piaq/piaq_release

ssh pi "cd piaq && docker-compose build piaq && docker system prune -f"