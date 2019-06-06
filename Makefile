target = armv7-unknown-linux-gnueabihf
mode = release
home = $(shell pwd)

.DEFAULT_GOAL = build

build:
	cargo build

linux:
	cargo build --examples --$(mode) --target=$(target) 

viz:
	cd $(home)/viz/viz && cargo build --$(mode) --target=$(target) 
	
upload: linux 
	scp $(home)/target/armv7-unknown-linux-gnueabihf/release/examples/linux pi@192.168.1.136:
	
.PHONY: deploy build ext viz
