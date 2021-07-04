.globl _start
.extern main

.text
.code32

_start:
    # We'll need a stack next, so set one up. The stack grows down from a high address.
    mov esp, offset boot.stack.high

    # The bootloader provides:
    #
    # * A magic number in EAX
    # * The physical address of the boot information record in EBX
    #
    # We clobber EAX and EBX, so save these values on the stack to pass to the Rust entrypoint later.
    push ebx
    push eax

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
    mov eax, offset boot.page_directory_pointer_table
    or eax, 0b11
    mov [boot.page_map_level_4_table], eax

    # Add one entry to PDPT pointing to the Page Directory Table.
    mov eax, offset boot.page_directory_table.0
    or eax, 0b11
    mov [boot.page_directory_pointer_table], eax

    # Populate the PDT with 512 entries, each pointing to a 2 MB physical page frame.
    #
    # We set an extra metadata bit, the Page Size Bit (0b10000000), to indicate that the PDTEs are
    # leaves and that the frames they point to are each 2 MB.
    mov ecx, 0
1:  mov eax, 0x200000
    mul ecx
    or eax, 0b10000011
    mov [boot.page_directory_table.0 + ecx * 8], eax
    inc ecx
    cmp ecx, 512
    jne 1b

    # Next we'll identity-map the LAPIC register file. It's in the 4 KB of physical memory starting
    # at 0xFEE00000, near the top of the 32-bit address space.

    # Populate the fourth PDPE. Point it at a new PDT.
    mov eax, offset boot.page_directory_table.3
    or eax, 0b11
    mov [boot.page_directory_pointer_table + 24], eax

    # Populate the 504th entry of the second PDT. Point to the 2 MB frame at 0xFEE00000.
    mov eax, 0xFEE00000
    or eax, 0b10000011
    mov [boot.page_directory_table.3 + 0xFB8], eax

    # Now we'll map the IOAPIC registers. They're at 0xFEC00000, one page before the LAPIC registers.

    # Populate the 503rd entry of the second PDT. Point it at the 2 MB frame at 0xFEC00000.
    mov eax, 0xFEC00000
    or eax, 0b10000011
    mov [boot.page_directory_table.3 + 0xFB0], eax

    # Load the PML4 Table.
    mov eax, offset boot.page_map_level_4_table
    mov cr3, eax

    # Enable physical address extension (PAE).
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    # Set EFER.LME, enabling (but not yet activating) long mode.
    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

    # Set CR0.PG and CR0.WP. Enable paging.
    #
    # Enabling paging while long mode is enabled has the effect of activating long mode,
    # but we remain in 32-bit compatibility mode until the far jump into Rust.
    mov eax, cr0
    or eax, 1 << 16
    or eax, 1 << 31
    mov cr0, eax

    # Load a 64-bit global descriptor table.
    lgdt [boot.global_descriptor_table.pointer]

    # Populate the segment registers (except CS).
    mov ax, offset boot.global_descriptor_table.data
    mov ss, ax
    mov ds, ax
    mov es, ax

    # Far-jump to the Rust kernel entrypoint, entering 64-bit mode in the process.
    #
    # A far jump is a jump across code segments. In this case, we're switching from the default
    # code segment the bootloader provided to our own, defined in the GDT below. Our code segment's
    # descriptor has its L bit set to indicate it contains 64-bit code, so jumping into it has the
    # effect of entering 64-bit mode.
    #
    # Pass the bootloader magic number and the address of the boot information record as parameters.
    # Rust declares main to use the C calling convention, so the first two parameters go in EDI and ESI.
    #
    # LLVM doesn't support absolute far jump syntax, perhaps because it's a vestige of the outdated
    # segmented memory model. If it did, the far jump would be as simple as this:
    #
    #   jmp boot.global_descriptor_table.code:main
    #
    # We work around this omission with retf.
    pop edi
    pop esi

    mov eax, offset boot.global_descriptor_table.code
    push eax

    mov eax, offset main
    push eax

    retf

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

boot.page_directory_table.0:
    .skip 4096

boot.page_directory_table.3:
    .skip 4096

boot.stack.low:
    .skip 4 * 4096
    .equ boot.stack.high, .


.section .rodata

# This is the early-boot global descriptor table, only used to switch into 64-bit long mode and
# call into the Rust kernel. The kernel replaces it immediately on boot.
boot.global_descriptor_table:
    .quad 0

# Kernel code segment
#
#   Base*: 0
#   Limit*: 0xFFFFF
#   Granularity*: 4 KB
#   Flags: Present (P), Readable (R), Long Mode (L)
#
# * Used in 32-bit compatibility mode, ignored in 64-bit long mode.
.equ boot.global_descriptor_table.code, . - boot.global_descriptor_table
    .quad (1 << 55) | (1 << 53) | (0xF << 48) | (1 << 47) | (1 << 44) | (1 << 43) | (1 << 41) | 0xFFFF

# Kernel data segment
#
#   Base*: 0
#   Limit*: 0xFFFFF
#   Granularity*: 4 KB
#   Stack Pointer Size*: 32 bits
#   Flags: Present (P), Readable (R)
#
# * Used in 32-bit compatibility mode, ignored in 64-bit long mode.
.equ boot.global_descriptor_table.data, . - boot.global_descriptor_table
    .quad (1 << 55) | (1 << 54) | (0xF << 48) | (1 << 47) | (1 << 44) | (1 << 41) | 0xFFFF

boot.global_descriptor_table.pointer:
    .word . - boot.global_descriptor_table - 1
    .quad boot.global_descriptor_table
