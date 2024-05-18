ENTRY(_start)
SECTIONS
{
    . = 0x80000;
    .text.boot : { *(.text.boot) }
    .text : { *(.text, .text.*) }
    .data : { *(.data, .data.*) }
    .rodata : { *(.rodata, .rodata.*) }
    .bss : { *(.bss, .bss.*) }

    . = ALIGN(8);
    . = . + 0x40000;
    LD_STACK_PTR0 = .;
    HEAP_START = .;
}
