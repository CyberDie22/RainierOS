;
; Copyright (c) 2025 Ben Buzard
; SPDX-License-Identifier: MPL-2.0
;

ORG 0x7C00
BITS 16

stage1:
    mov ah, 0xE
    mov al, 66
    mov bh, 0x0
    int 0x10
    mov al, 73
    int 0x10
    mov al, 79
    int 0x10
    mov al, 83
    int 0x10
    mov al, 32
    int 0x10
    mov al, 87
    int 0x10
    mov al, 111
    int 0x10
    mov al, 114
    int 0x10
    mov al, 107
    int 0x10
    mov al, 115
    int 0x10
    mov al, 33
    int 0x10
.halt:
    hlt
    jmp .halt

times 510-($-$$) db 0
dw 0xAA55
