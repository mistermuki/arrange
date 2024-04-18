HW_DIR = ./src/hw

.PHONY = build bitstream release run run-log clean

bitstream:
	# Ensures we always have the freshest bitstream.
	cd $(HW_DIR) && $(MAKE) bitstream

build: bitstream 
	cargo build 

release: bitstream
	cargo build --release

run: bitstream
	cargo run

run-log: bitstream
	RUST_LOG=debug cargo run

clean:
	cd $(HW_DIR) && make clean
	cargo clean
