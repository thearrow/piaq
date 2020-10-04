#!/bin/bash

# updates
apt update
apt upgrade --yes

# docker
if ! type "docker" > /dev/null; then
    curl -fsSL https://get.docker.com -o get-docker.sh
    bash get-docker.sh
    usermod -aG docker ubuntu
else 
    echo "docker already installed. skipping."
fi

# utilities
apt install --yes i2c-tools htop