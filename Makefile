BIND      ?= 192.168.1.125
BROADCAST ?= 192.168.1.255:60000
LISTEN    ?= 192.168.1.125:60001

CLI = cd examples/cli && cargo fmt && RUST_LOG=debug cargo run -- --debug --bind $(BIND) --broadcast $(BROADCAST) --listen $(LISTEN)

clean:
	cargo clean

generate:
	cd .codegen && make generate

update:
	cargo update

format:
	cargo fmt --package uhppoted

build: format
	mkdir -p uhppoted/src/generated
	cargo build --package uhppoted

test: 
	cargo test --lib

lint:
	cargo clippy

integration-tests: build
	cargo test

build-all: build test lint integration-tests
	cd examples/cli && cargo fmt && cargo build

build-github: build test lint
	cd examples/cli && cargo fmt && cargo build

release:
	@echo "Nothing to do

package: release
	@echo "Nothing to do

publish: release
	@echo "Nothing to do

debug: build
	$(CLI) find-controllers

find-controllers: build
	$(CLI) find-controllers
