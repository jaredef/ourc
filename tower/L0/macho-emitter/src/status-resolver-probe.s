.text
.globl _main
.p2align 2
_main:
  sub sp, sp, #1, lsl #12
  mov x19, sp

  mov x16, #3
  mov x0, #0
  mov x1, x19
  mov x2, #4096
  svc #0x80
  add x20, x19, x0

  mov x0, x19
  mov x1, x20
  adr x2, pat_chained
  mov x3, #13
  bl contains
  cbnz x0, status_chained

  mov x0, x19
  mov x1, x20
  adr x2, pat_renamed
  mov x3, #13
  bl contains
  cbnz x0, status_renamed

  mov x0, x19
  mov x1, x20
  adr x2, pat_sub
  mov x3, #13
  bl contains
  cbnz x0, status_sub

  mov w22, #11
  b write_status

status_chained:
  mov w22, #7
  b write_status

status_renamed:
  mov w22, #12
  b write_status

status_sub:
  mov w22, #38
  b write_status

write_status:
  strb w22, [x19]
  mov x16, #4
  mov x0, #1
  mov x1, x19
  mov x2, #1
  svc #0x80

  mov x16, #1
  mov x0, #0
  svc #0x80

contains:
  sub x4, x1, x3
scan_next:
  cmp x0, x4
  b.hi scan_no
  mov x5, #0
scan_compare:
  ldrb w6, [x0, x5]
  ldrb w7, [x2, x5]
  cmp w6, w7
  b.ne scan_advance
  add x5, x5, #1
  cmp x5, x3
  b.ne scan_compare
  mov x0, #1
  ret
scan_advance:
  add x0, x0, #1
  b scan_next
scan_no:
  mov x0, #0
  ret

pat_chained:
  .ascii "v3: i32 = sub"
pat_renamed:
  .ascii "v8: i32 = add"
pat_sub:
  .ascii "v2: i32 = sub"
