target = armv7-unknown-linux-gnueabihf
mode = release
home = $(shell pwd)

.DEFAULT_GOAL = build

build:
	cargo build

ext:
	cd $(home)/example && cargo build --$(mode) --target $(target) 

viz:
	cd $(home)/viz/viz && cargo build --$(mode) --target $(target) 
	
deploy: ext
	scp $(home)/example/target/$(target)/$(mode)/example pi@192.168.1.136:
	
.PHONY: deploy build ext viz
