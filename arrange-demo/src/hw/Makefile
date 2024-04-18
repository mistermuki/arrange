# Project setup
PROJ      = bitstream
BUILD     = ./build
DEVICE    = 1k
FOOTPRINT = tq144 

# Files
FILES = top.v clock_divider.v

.PHONY: all bitstream clean burn timing

all:  $(BUILD)/$(PROJ).blif $(BUILD)/$(PROJ).bin

bitstream: $(BUILD)/$(PROJ).bin

$(BUILD):
	mkdir -p $(BUILD)

$(BUILD)/$(PROJ).blif: $(BUILD) $(FILES)
	# synthesize using Yosys
	echo "Running Yosys for BLIF"
	yosys -p "synth_ice40 -top top -blif $(BUILD)/$(PROJ).blif" $(FILES)

$(BUILD)/$(PROJ).json: $(BUILD) $(FILES)
	# synthesize using Yosys
	echo "Running Yosys for JSON"
	yosys -p "synth_ice40 -top top -json $(BUILD)/$(PROJ).json" $(FILES)

$(BUILD)/$(PROJ).asc: $(BUILD) $(FILES) $(BUILD)/$(PROJ).json
	# Place and route using arachne
	#arachne-pnr -d $(DEVICE) -P $(FOOTPRINT) -o $(BUILD)/$(PROJ).asc -p pinmap.pcf $(BUILD)/$(PROJ).blif
	nextpnr-ice40 --hx$(DEVICE) --json $(BUILD)/$(PROJ).json --pcf pinmap.pcf --asc $(BUILD)/$(PROJ).asc

$(BUILD)/$(PROJ).bin: $(BUILD)/$(PROJ).asc
	icepack $(BUILD)/$(PROJ).asc $(BUILD)/$(PROJ).bin


burn: $(BUILD)/$(PROJ).bin
	# iceprog $(BUILD)/$(PROJ).bin
	#
	# use arrange-iceprog instead of iceprog!
	arrange-iceprog $(BUILD)/$(PROJ).bin -v

timing: $(BUILD)/$(PROJ).asc
	icetime -tmd hx$(DEVICE) $(BUILD)/$(PROJ).asc

clean:
	rm build/*
