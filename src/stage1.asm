;
; Copyright (c) 2025 Ben Buzard
; SPDX-License-Identifier: MPL-2.0
;

ORG 0x7C00
BITS 16

stage1:
    hlt
    jmp stage1

times 510-($-$$) db 0
dw 0xAA55
