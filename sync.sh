#!/bin/bash

rsync -rauLhP --exclude=.git --exclude=.vscode --exclude=target . pi:/home/ubuntu/piaq