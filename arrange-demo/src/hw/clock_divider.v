// HARDWARE ONLY - not to be used in simulation
module clock_divider (
    clock,
    divided_clocks
);
  input wire clock;
  output wire [31:0] divided_clocks;

  initial divided_clocks = 0;

  always @(posedge clock) divided_clocks <= divided_clocks + 1;

endmodule

