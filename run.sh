#!/bin/bash

cross build --target armv7-unknown-linux-musleabihf

if [ $? -eq 0 ]
then
    echo "compilation succeeded, uploading..."
else
    echo "compilation failed! exiting..."
    exit 1
fi

rsync -azvhP target/armv7-unknown-linux-musleabihf/debug/piaq pi:/home/ubuntu/piaq

ssh pi "sudo ./piaq/piaq"