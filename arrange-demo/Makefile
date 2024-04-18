HW_DIR = ./src/hw

build:
	cd $(HW_DIR) && make bitstream
	cargo build 

release:
	cd $(HW_DIR) && make bitstream
	cargo build --release

run: 
	cd $(HW_DIR) && make bitstream
	cargo run

clean:
	$(MAKE) -C $(HW_DIR) clean
	cargo clean
