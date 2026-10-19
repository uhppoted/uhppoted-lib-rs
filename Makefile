BIND      ?= 192.168.1.125
BROADCAST ?= 192.168.1.255:60000
LISTEN    ?= 192.168.1.125:60001

CLI = cd examples/cli && cargo fmt && cargo run -- --debug --bind $(BIND) --broadcast $(BROADCAST) --listen $(LISTEN)

clean:
	cargo clean

generate:
	@echo "Nothing to do

update:
	@echo "Nothing to do

format:
	cargo fmt --package uhppoted

build: format
	cargo build --package uhppoted

test:
	@echo "Nothing to do

lint:
	@echo "Nothing to do

integration-tests: build
	@echo "Nothing to do

build-all: build
	cd examples/cli && cargo fmt && cargo build

release:
	@echo "Nothing to do

package: release
	@echo "Nothing to do

publish: release
	@echo "Nothing to do

debug: build
	$(CLI) find-controllers
