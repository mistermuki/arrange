// Blink an LED provided an input clock
/* module */
module top (
    CLK_12_MHZ,
    led1,
    led2,
    led3,
    led4
);
  /* I/O */
  input CLK_12_MHZ;
  output led1;
  output led2;
  output led3;
  output led4;

  /* Counter register */
  wire [31:0] clk;
  // 0 is 12 mHz, 1 is 6 mHz, 2 is 3 mHz.
  clock_divider cdiv (
      .clock(CLK_12_MHZ),
      .divided_clocks(clk)
  );

  /* LED drivers */
  reg led1_value = 1;
  reg led2_value = 0;
  reg led3_value = 1;
  reg led4_value = 0;
  assign led1 = led1_value;
  assign led2 = led2_value;
  assign led3 = led3_value;
  assign led4 = led4_value;

  /* always */
  always @(posedge clk[23]) begin
    led4_value <= led3_value;
    led3_value <= led2_value;
    led2_value <= led1_value;
    led1_value <= led1_value + 1;
  end

endmodule
