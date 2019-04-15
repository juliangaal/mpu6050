#!/usr/bin/env bash
sudo apt-get update && sudp apt-get upgrade
sudo apt-get install git python3-pip tmux vim -y
pip install smbus
git clone https://github.com/juliangaal/mpu6050.git
