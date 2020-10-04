#!/bin/bash

cross build --target armv7-unknown-linux-musleabihf

rsync -azvhP target/armv7-unknown-linux-musleabihf/debug/piaq pi:/home/ubuntu/piaq

ssh pi "./piaq/piaq"