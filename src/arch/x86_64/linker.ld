ENTRY(start)

SECTIONS {
    . = 1M; /* Start at 1 mb in RAM */

    .boot : {
        /* Make sure the multiboot header is at the beginning of the file */
        KEEP(*(.multiboot_header))
    }

    .text : {
        *(.text)
    }
}
