target = armv7-unknown-linux-gnueabihf
cross_comp_packages = gcc-arm-linux-gnueabihf libc6-armhf-cross libc6-dev-armhf-cross
mode = release
home = $(shell pwd)

.DEFAULT_GOAL = build

build:
	cargo build

test:
	cargo test

rustup_cross:
	rustup target add $(target)

install:
	apt-get install $(cross_comp_packages)

linux:
	cargo build --examples --$(mode) --target=$(target) 

viz:
	cd $(home)/viz/viz && cargo build --$(mode) --target=$(target) 
	
upload: linux 
	scp $(home)/target/armv7-unknown-linux-gnueabihf/release/examples/linux pi@192.168.1.136:
	
.PHONY: deploy build ext viz
