#!/bin/bash

rsync -rauLhP --exclude=.git --exclude=.vscode --exclude=.idea --exclude=target . pi:/home/ubuntu/piaq
