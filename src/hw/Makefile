# iCEStick (HX1k)
PROJ      = arrange
BUILD     = ./build
DEVICE    = 1k
FOOTPRINT = tq144 

# Files
FILES = top.v

.PHONY: all clean burn timing

all $(BUILD)/$(PROJ).asc $(BUILD)/$(PROJ).bin:
	# if build folder doesn't exist, create it
	mkdir -p $(BUILD)
	# synthesize using Yosys
	yosys -p "synth_ice40 -top top -blif $(BUILD)/$(PROJ).blif -json $(BUILD)/$(PROJ).json" $(FILES)
	# Place and route using nextpnr
	nextpnr-ice40 --hx$(DEVICE) --package $(FOOTPRINT) --json build/$(PROJ).json --pcf pinmap.pcf --asc build/$(PROJ).asc
	# Convert to bitstream using IcePack
	icepack $(BUILD)/$(PROJ).asc $(BUILD)/$(PROJ).bin

burn: $(BUILD)/$(PROJ).bin
	iceprog $(BUILD)/$(PROJ).bin

timing: $(BUILD)/$(PROJ).asc
	icetime -tmd hx$(DEVICE) $(BUILD)/$(PROJ).asc

clean:
	rm build/*
