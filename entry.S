 
.globl _start

// Entry point for the kernel. Registers:
// x0 -> 32 bit pointer to DTB in memory (primary core only) / 0 (secondary cores)
// x1 -> 0
// x2 -> 0
// x3 -> 0
// x4 -> 32 bit kernel entry point, _start location
.section .text.boot, "ax"
_start:
        // set stack before our code
        ldr     x5, =_start
        mov     sp, x5

        // Identify primary cpu and halt the rest
        // This seems to be needed in QEMU.
        // In the real RPi3 all cores except 0
        // are halted before reaching the _start symbol.
        mrs x6, MPIDR_EL1
        and x6, x6, #0x3
        cbz x6, primary_cpu
        b inf_loop
 
primary_cpu:
clear_bss:
        ldr     x5, =__bss_start
        ldr     w6, =__bss_size
bss_loop:
        cbz     w6, start_kernel
        str     xzr, [x5], #8
        sub     w6, w6, #1
        cbnz    w6, bss_loop
 
start_kernel:
        // jump to Rust code, should not return
        bl      kernel_main

inf_loop:
        b inf_loop
