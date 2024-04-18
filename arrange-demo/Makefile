HW_DIR = ./src/hw

all:
	$(MAKE) -C $(HW_DIR)
	cargo build 

run: 
	$(MAKE) -C $(HW_DIR) all
	cargo run

clean:
	$(MAKE) -C $(HW_DIR) clean
	cargo clean
