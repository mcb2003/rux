global start
extern long_mode_start

section .text
bits 32

start:
    ; Set the stack pointer
    mov esp, stack_top
    ; Move Multiboot info struct pointer to edi
    mov edi, ebx

    ; Sanity checks
    call check_multiboot
    call check_cpuid
    call check_long_mode
    
    ; Setup the page tables and enable paging
    call setup_paging
    call enable_paging

    ; Load the 64 bit GDT
    lgdt [gdt64.pointer]
    ; Far-jump, reloading cs and entering long mode!
    jmp gdt64.code:long_mode_start

; Displays an error code on the screen and halts indefinitely
; edsishould contain a pointer to a null-terminated error message
error:
; Print "ERROR: "
    mov dword [0xb8000], 0x04520445
    mov dword [0xb8004], 0x044f0452
    mov dword [0xb8008], 0x043a0452
    mov word [0xb800c], 0x0420
    mov edi, 0xb800e
    mov ah, 0x04 ; Set the color attributes
    ; Print the null-terminated string
    .output:
    lodsb ; Load next character
    ; Stop at null terminator
    and al, al
    jz .halt
    stosw ; Output one character
    jmp .output
    cli
    .halt:
    hlt
    jmp .halt

; Checks if we were started from a Multiboot2 bootloader
check_multiboot:
    cmp eax, 0x36d76289
    jne .no_multiboot
    ret
    .no_multiboot:
        mov esi, no_multiboot_msg
        jmp error

; Check if CPUID is supported by attempting to flip the ID bit (bit 21)
; in the FLAGS register.
check_cpuid:
    pushfd
    pop eax
    mov ecx, eax
    ; Flip the ID bit
    xor eax, 1 << 21
; Try to reload modified flags
    push eax
    popfd
    ; Copy FLAGS back to eax (with the flipped bit if CPUID is supported)
    pushfd
    pop eax
    ; Restore FLAGS from the old version stored in ECX (i.e. flipping the
    ; ID bit back if it was ever flipped).
    push ecx
    popfd
; Check if the bit was successfully flipped
    cmp eax, ecx
    je .no_cpuid
    ret
    .no_cpuid:
       mov esi, no_cpuid_msg
        jmp error

; Check whether the CPU supports the 64 bit long mode
check_long_mode:
; Get the highest supported cpuid argument
    mov eax, 0x80000000    ; implicit argument for cpuid
    cpuid
    cmp eax, 0x80000001
    jb .no_long_mode       ; the CPU is too old for long mode
    ; use extended info to test if long mode is available
    mov eax, 0x80000001    ; extended processor info
    cpuid                  ; returns various feature bits in ecx and edx
    test edx, 1 << 29      ; LM bit
    jz .no_long_mode
    ret
    .no_long_mode:
        mov esi, no_long_mode_msg
        jmp error

; Identity map the first 1 GiB of the kernel and setup paging
setup_paging:
; Recursively map the last p4 entry to the p4 table itself
mov eax, p4_table
    or eax, 0b11 ; present | writable
    mov [p4_table + 511 * 8], eax
    ; Map the first p4 entry to the p3 table
    mov eax, p3_table
    or eax, 0b11 ; present | writable
    mov [p4_table], eax
    ; Map the first p3 entry to the p2 table
    mov eax, p2_table
    or eax, 0b11 ; present | writable
    mov [p3_table], eax
        ; map each P2 entry to a huge 2MiB page
    mov ecx, 511
    .map_p2_table:
        ; map ecx-th P2 entry to a huge page that starts at address 2MiB*ecx
        mov eax, 0x200000  ; 2MiB
        mul ecx            ; start address
        or eax, 0b10000011 ; present | writable | hugepg
        mov [p2_table + ecx * 8], eax

        dec ecx
        jz .map_p2_table
    ret

; Enable the newly set up page-tables
enable_paging:
    ; Load p4 address to cr3
    mov eax, p4_table
    mov cr3, eax
    ; enable PAE in cr4
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax
    ; set the long mode bit in the EFER MSR
    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8 ; LM bit
    wrmsr
    ; enable paging in the cr0 register
    mov eax, cr0
    or eax, 1 << 31 ; PG bit
    mov cr0, eax
    ret

section .bss
align 4096

; Page tables
p4_table:
    resb 4096
p3_table:
    resb 4096
p2_table:
    resb 4096

    ; Kernel stack
stack_bottom:
    resb 4096 * 4
stack_top:

section .rodata
; Error messages
no_multiboot_msg: db "Must be booted with a multiboot bootloader", 0
no_cpuid_msg: db "The CPU does not support CPUID", 0
no_long_mode_msg: db "The CPU is not x86_64 compatible", 0

; The 64 bit Global Descriptor Table
gdt64:
    dq 0 ; zero entry
    .code: equ $ - gdt64
    dq (1<<43) | (1<<44) | (1<<47) | (1<<53) ; code segment
    .pointer:
        dw $ - gdt64 - 1 ; Limit (size - 1)
        dq gdt64; Address
