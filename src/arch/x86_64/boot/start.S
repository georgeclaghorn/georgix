.globl start
.extern main

.text
.code32

start:
    # The bootloader provides:
    #
    # * A magic number in EAX
    # * The physical address of the boot information record in EBX
    #
    # We clobber EAX and EBX, so save these values in EDI and ESI to pass to the Rust entrypoint later.
    movl %eax, %edi
    movl %ebx, %esi

    # The bootloader leaves the CPU in 32-bit protected mode. We want to switch into 64-bit long
    # mode before jumping into Rust. This requires that we activate paging.
    #
    # Setting up 32-bit page tables is the first step. For now, we'll identity-map virtual memory
    # to physical memory. The kernel will load different page tables later for true virtualization.

    # Add one entry to the level 3 page table pointing to the level 2 page table.
    #
    # Notice that we add 3 (0b11) to the level 2 page table's address. Addresses in page table
    # entries are required to be aligned to 4-kilobyte boundaries. This means that the address is
    # always a multiple of 4096, which in turn means that the lower 12 bits are always zero. Since
    # the lower 12 bits are always zero, they can be used to store metadata in page table entries.
    # We set metadata bits indicating the level 2 page table is present (0b1) and writable (0b10).
    movl $(page_table + 0x1000), %eax
    orl $0b11, %eax
    movl %eax, (page_table)

    # Add one entry to the level 2 page table pointing to the level 1 page table.
    movl $(page_table + 0x2000), %eax
    orl $0b11, %eax
    movl %eax, (page_table + 0x1000)

    # Populate the level 1 page table with 512 entries, each pointing to a contiguous 2 MB block
    # of physical memory. We set an extra metadata bit, the Page Size Bit (0b10000000), to indicate
    # that these page table entries are leaves and that the page frames they address are each 2 MB.
    movl $0, %ecx
1:  movl 0x200000, %eax
    mull %ecx
    orl $0b10000011, %eax
    mov %eax, page_table + 0x2000(,%ecx,8)
    inc %ecx
    cmpl $512, %ecx
    jne 1b

    # Load the page table.
    movl page_table, %eax
    movl %eax, %cr3

    # Enable physical address extension (PAE).
    movl %cr4, %eax
    orl $(1 << 5), %eax
    movl %eax, %cr4

    # Set EFER.LME, enabling (but not yet activating) long mode.
    movl $0xC0000080, %ecx
    rdmsr
    orl $(1 << 8), %eax
    wrmsr

    # Set CR0.PG and CR0.WP. Enable paging.
    #
    # Enabling paging while long mode is enabled has the effect of activating long mode,
    # but we remain in 32-bit compatibility mode until the far jump into Rust.
    movl %cr0, %eax
    orl $(1 << 31 | 1 << 16), %eax
    movl %eax, %cr0

    # Load a 64-bit global descriptor table.
    lgdt (global_descriptor_table.pointer)

    # Populate the segment registers (except CS).
    movw $(global_descriptor_table.data), %ax
    movw %ax, %ss
    movw %ax, %ds
    movw %ax, %es

    # Far-jump to the Rust kernel entrypoint, entering 64-bit mode in the process.
    #
    # Pass the bootloader magic number and the address of the boot information record. We moved
    # them into the EDI and ESI argument registers earlier.
    ljmp $(global_descriptor_table.code), $(main)

    # The kernel shouldn't return, but if it does for some reason, park.
1:  hlt
    jmp 1b


# We rely on the BSS section being entirely zeroed out. It would normally be our responsibility as
# the operating system to zero it out. However, the Multiboot 2 standard requires the bootloader
# to do so before transferring control to the kernel.
.bss
.align 4096

page_table:
    # Reserve space for three levels of page tables.
    #
    # Each level must be aligned on a 4-kilobyte boundary. The .align assembler directive above
    # ensures that the top level, the third, is properly aligned. A level has at most 512 entries,
    # each 8 bytes long, and so uses a maximum of 4 kilobytes. How convenient is that?
    .skip 4096 * 3


.section .rodata

global_descriptor_table:
    .quad 0

.equ global_descriptor_table.code, . - global_descriptor_table
    .quad (1 << 53) | (1 << 47) | (1 << 44) | (1 << 43) | (1 << 41)

.equ global_descriptor_table.data, . - global_descriptor_table
    .quad (1 << 47) | (1 << 44) | (1 << 41)

global_descriptor_table.pointer:
    .word . - global_descriptor_table - 1
    .quad global_descriptor_table
