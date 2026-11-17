BIND      ?= 192.168.1.125:0
BROADCAST ?= 192.168.1.255:60000
LISTEN    ?= 192.168.1.125:60001

CLI = cd examples/cli && cargo fmt && RUST_LOG=debug cargo run -- --debug --bind $(BIND) --broadcast $(BROADCAST) --listen $(LISTEN)

CONTROLLER ?= 405419896
DEST ?= 192.168.1.125:60000

ADDRESS ?= 192.168.1.125
NETMASK ?= 255.255.255.0
GATEWAY ?= 192.168.1.1
LISTENER ?= 192.168.1.125:60001
DOOR ?= 3
MODE ?= 2
DELAY ?= 11
PASSCODES ?= 12345,54321,987654

clean:
	cargo clean

generate:
	mkdir -p uhppoted/src/codec/generated
	mkdir -p uhppoted/tests/generated
	cd .codegen && make generate

update:
	cargo update

format:
	cargo fmt --package uhppoted
	rustfmt uhppoted/tests/generated/*
	rustfmt uhppoted/src/codec/generated/*

build: format
	cargo build --package uhppoted

test: 
	cargo test --lib

lint:
	cargo clippy

integration-tests: build
	cargo test --test integration_tests -- --nocapture

build-all: build test lint integration-tests
	cd examples/cli && cargo fmt && cargo build

release:
	@echo "Nothing to do

package: release
	@echo "Nothing to do

publish: release
	@echo "Nothing to do

debug: generate
	$(CLI) get-controller --controller $(CONTROLLER)
	$(CLI) get-controller --controller $(CONTROLLER) --dest $(DEST)
	$(CLI) get-controller --controller $(CONTROLLER) --dest $(DEST) --udp
	$(CLI) get-controller --controller $(CONTROLLER) --dest $(DEST) --tcp

usage: build
	$(CLI)

find-controllers: build
	$(CLI) find-controllers

get-controller: build
	$(CLI) get-controller --controller $(CONTROLLER)
# 	$(CLI) get-controller --controller $(CONTROLLER) --dest $(DEST)
# 	$(CLI) get-controller --controller $(CONTROLLER) --dest $(DEST) --udp
# 	$(CLI) get-controller --controller $(CONTROLLER) --dest $(DEST) --tcp

set-IPv4: build
	$(CLI) set-IPv4 --controller $(CONTROLLER)                      --address $(ADDRESS) --netmask $(NETMASK) --gateway $(GATEWAY)
# 	$(CLI) set-IPv4 --controller $(CONTROLLER) --dest $(DEST)       --address $(ADDRESS) --netmask $(NETMASK) --gateway $(GATEWAY)
# 	$(CLI) set-IPv4 --controller $(CONTROLLER) --dest $(DEST) --udp --address $(ADDRESS) --netmask $(NETMASK) --gateway $(GATEWAY)
# 	$(CLI) set-IPv4 --controller $(CONTROLLER) --dest $(DEST) --tcp --address $(ADDRESS) --netmask $(NETMASK) --gateway $(GATEWAY)

get-time: build
	$(CLI) get-time --controller $(CONTROLLER)

set-time: build
	$(CLI) set-time --controller $(CONTROLLER)

get-listener: build
	$(CLI) get-listener --controller $(CONTROLLER)

set-listener: build
	$(CLI) set-listener --controller $(CONTROLLER) --address $(LISTENER)
	$(CLI) set-listener --controller $(CONTROLLER) --address $(LISTENER) --interval 30

get-door: build
	$(CLI) get-door --controller $(CONTROLLER) --door $(DOOR)

set-door: build
	$(CLI) set-door --controller $(CONTROLLER) --door $(DOOR) --mode $(MODE) --delay $(DELAY)

set-door-passcodes: build
	$(CLI) set-door-passcodes --controller $(CONTROLLER) --door $(DOOR) --passcodes $(PASSCODES)

open-door: build
	$(CLI) open-door --controller $(CONTROLLER) --door $(DOOR)

get-status: build
	$(CLI) get-status --controller $(CONTROLLER)
