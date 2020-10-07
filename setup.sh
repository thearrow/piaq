#!/bin/bash

# updates
apt update
apt upgrade --yes
apt autoremove --yes

# utilities
apt install --yes i2c-tools htop python3 python3-pip

# docker
if ! type "docker" > /dev/null; then
    curl -fsSL https://get.docker.com -o get-docker.sh
    bash get-docker.sh
    usermod -aG docker ubuntu
else 
    echo "docker already installed. skipping."
fi

# docker-compose
if ! type "docker-compose" > /dev/null; then
    pip3 install docker-compose
else 
    echo "docker-compose already installed. skipping."
fi
