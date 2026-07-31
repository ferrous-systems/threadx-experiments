MEMORY {
  /* Nonecure Code uses the first flash bank, nonsecure alias */
  FLASH (rx): ORIGIN = 0x08000000, LENGTH = 2M
  /* Nonecure Code uses SRAM3, nonsecure alias */
  RAM (rwx) : ORIGIN = 0x200D0000, LENGTH = 832K
}

/* Link to the raw assembly function, avoiding a Rust function prologue/epilogue */
PROVIDE(PendSV = PendSV_Handler);
