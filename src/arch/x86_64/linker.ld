SECTIONS {
    . = 1M;

    .text : {
      KEEP(*(.text.multiboot))
      *(.text)
    }

    .bss : { *(.bss) }
    .rodata : { *(.rodata) }
}
