#!/bin/bash

rsync -rhP --exclude=.git --exclude=.vscode . pi:/home/ubuntu/piaq