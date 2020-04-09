# Multiboot 2 header
.section .text.multiboot, "a"
.align 8

.Lstart:
    .long 0xe85250d6       # Magic number
    .long 0                # Architecture (0 = x86 in 32-bit protected mode)
    .long .Lend - .Lstart  # Header length

    # Checksum: must sum to zero with the above fields.
    .long -(0xe85250d6 + 0 + (.Lend - .Lstart))

    # No other tags are needed here because the kernel image is an ELF executable. The bootloader
    # can glean the information it needs–like the size of the kernel image, the kernel entrypoint,
    # and the locations of various executable sections–from the ELF header.

    # End tag
    .word 0  # Type (0 = end)
    .word 0  # Fixed 0
    .long 8  # Length
.Lend:
