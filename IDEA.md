ARRANGE: Adaptive Accelerators

iCE40 is the most realistic utilizing YoSYS infra.

Could also use Xilinx or Quartus but would have to send the HDL up (to a cloud service) to get synthesized
and send the bitstream back. This would have to be intergrated into the build system.

Example Board:
https://digilent.com/shop/arty-a7-100t-artix-7-fpga-development-board/

Example iCEStick:
https://www.latticesemi.com/icestick#_128A360799DA425689D8C242595BAB56

Graphics cards are fast, but we can be faster by optimizing for our domain!

Either USB/PCIE/Ethernet.
    - This allows for the usage of ARRANGE Accelerators in embedded systems.

Write your program in a systems language:
    - Rust (Focus on this!)

Then, you can write your own program accelerator in an HDL (like SystemVerilog)! We will then synthesis the HDL for your specific accelerator (or include multiple bitstreams?)

The idea is that you write your program (or library), you write your own description of hardware for the accelerator you want.
    - It would be nice if this was typesafe (with Rust? Maybe a DSL for describing the hardware using premade elements for ease)

 When your program starts, it is able to operate in either "STANDARD" or "ARRANGE" mode depending on if the AA is connected (and compatible with the embedded bitstream).
    - If in ARRANGE mode, we write the bitstream to the FPGA and begin operating (communicating with it as needed).
        - During the arbitration, if the FPGA already contains the bitstream you are going to write, it will not rewrite.
            - PC will ask the ARRANGE card for ID, status, etc etc.
            - ARRANGE card will reply with ID, status, hash of loaded bitstream etc.
            - If hashes differ, reflash. Otherwise, continue execution.
            
    - If in NORMAL mode, everything happens on the CPU as normally expected.

Downsides:
    - Binary size due to embedded bitstream.
    - Occasional startup time can be high due to FPGA reflashing.

Upsides:
    - Certain operations can see significant speed ups due to mass parallelization of algorithms.
        - Compression, Encryption?, etc
        - Maybe port a solver (like FEM?) to utilize hardware?
