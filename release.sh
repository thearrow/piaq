#!/bin/bash

RPI_TARGET=armv7-unknown-linux-musleabihf

if cross build --release --target $RPI_TARGET; then
    echo "compilation succeeded, uploading..."
else
    echo "compilation failed! exiting..."
    exit 1
fi

rsync -azvhP target/$RPI_TARGET/release/piaq pi:/home/ubuntu/piaq/piaq_release

ssh pi "cd piaq && docker-compose build piaq && docker-compose up -d && docker system prune -f"
