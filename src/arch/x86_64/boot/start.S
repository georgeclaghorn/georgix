.globl _start
.extern main

.text
.code32

_start:
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
    # Setting up page tables is the first step. For now, we'll identity-map virtual memory to
    # physical memory. The kernel will load different page tables later for true virtualization.

    # Add one entry to the Page Map Level 4 Table pointing to the Page Directory Pointer Table.
    #
    # Notice that we add 3 (0b11) to the PDPT address. Addresses in page table entries are required
    # to be aligned to 4-kilobyte boundaries. This means that the address is always a multiple of
    # 4096, which in turn means that the lower 12 bits are always zero. Since the lower 12 bits are
    # always zero, they can be used to store metadata in page table entries. We set metadata bits
    # indicating the PDPT is present (0b1) and writable (0b10).
    movl $boot.page_directory_pointer_table, %eax
    orl $0b11, %eax
    movl %eax, boot.page_map_level_4_table

    # Add one entry to the PDPT pointing to the Page Directory Table.
    movl $boot.page_directory_table, %eax
    orl $0b11, %eax
    movl %eax, boot.page_directory_pointer_table

    # Populate the PDT with 512 entries, each pointing to a 2 MB physical page frame.
    #
    # We set an extra metadata bit, the Page Size Bit (0b10000000), to indicate that the PDTEs are
    # leaves and that the frames they point to are each 2 MB.
    movl $0, %ecx
1:  movl 0x200000, %eax
    mull %ecx
    orl $0b10000011, %eax
    movl %eax, boot.page_directory_table(,%ecx,8)
    inc %ecx
    cmpl $512, %ecx
    jne 1b

    # Next we'll identity-map the LAPIC register file. It's in the 4 KB of physical memory starting
    # at 0xFEE00000, near the top of the 32-bit address space.

    # Populate the fourth PDPE. Point it at a second PDT.
    movl $(boot.page_directory_table + 0x1000), %eax
    orl $0b11, %eax
    movl %eax, boot.page_directory_pointer_table + 24

    # Populate the 504th entry of the second PDT. Point to the 2 MB frame at 0xFEE00000.
    movl $0xFEE00000, %eax
    orl $0b10000011, %eax
    movl %eax, boot.page_directory_table + 0x1FB8

    # Load the PML4 Table.
    movl $boot.page_map_level_4_table, %eax
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
    lgdt boot.global_descriptor_table.pointer

    # Populate the segment registers (except CS).
    movw $(boot.global_descriptor_table.data), %ax
    movw %ax, %ss
    movw %ax, %ds
    movw %ax, %es

    # Far-jump to the Rust kernel entrypoint, entering 64-bit mode in the process.
    #
    # A far jump is a jump across code segments. In this case, we're switching from the default
    # code segment the bootloader provided to our own, defined in the GDT below. Our code segment's
    # descriptor has its L bit set to indicate it contains 64-bit code, so jumping into it has the
    # effect of entering 64-bit mode.
    #
    # Pass the bootloader magic number and the address of the boot information record. We moved
    # them into the EDI and ESI argument registers earlier.
    ljmp $(boot.global_descriptor_table.code), $(main)

    # The kernel shouldn't return, but if it does for some reason, park.
1:  hlt
    jmp 1b


# We rely on the BSS section being entirely zeroed out. It would normally be our responsibility as
# the operating system to zero it out. However, the Multiboot 2 standard requires the bootloader
# to do so before transferring control to the kernel.
.bss
.align 4096

boot.page_map_level_4_table:
    .skip 4096

boot.page_directory_pointer_table:
    .skip 4096

boot.page_directory_table:
    .skip 2 * 4096


.section .rodata

boot.global_descriptor_table:
    .quad 0

.equ boot.global_descriptor_table.code, . - boot.global_descriptor_table
    .quad (1 << 53) | (1 << 47) | (1 << 44) | (1 << 43) | (1 << 41)

.equ boot.global_descriptor_table.data, . - boot.global_descriptor_table
    .quad (1 << 47) | (1 << 44) | (1 << 41)

boot.global_descriptor_table.pointer:
    .word . - boot.global_descriptor_table - 1
    .quad boot.global_descriptor_table
