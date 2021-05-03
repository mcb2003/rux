global start

section .multiboot_header
header_start:
    dd 0xe85250d6                ; Multiboot2 magic number
    dd 0                         ; architecture (i386)
    dd header_end - header_start ; header length
    ; checksum
    dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))

    ; end tag
    dw 0    ; type
    dw 0    ; flags
    dd 8    ; size
header_end:

section .text
bits 32
start:
    ; Print "OK" to the VGA text buffer
    mov dword [0xb8000], 0x2f4b2f4f
    hlt
