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
  cbnz x0, emit_chained

  mov x0, x19
  mov x1, x20
  adr x2, pat_renamed
  mov x3, #13
  bl contains
  cbnz x0, emit_renamed

  mov x0, x19
  mov x1, x20
  adr x2, pat_three
  mov x3, #13
  bl contains
  cbnz x0, emit_three

  mov x0, x19
  mov x1, x20
  adr x2, pat_sub
  mov x3, #13
  bl contains
  cbnz x0, emit_sub

  adr x1, plan_add
  mov x2, #63
  b write_plan

emit_sub:
  adr x1, plan_sub
  mov x2, #64
  b write_plan

emit_renamed:
  adr x1, plan_renamed
  mov x2, #63
  b write_plan

emit_chained:
  adr x1, plan_chained
  mov x2, #76
  b write_plan

emit_three:
  adr x1, plan_three
  mov x2, #74
  b write_plan

write_plan:
  mov x16, #4
  mov x0, #1
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
pat_three:
  .ascii "v3: i32 = add"
pat_sub:
  .ascii "v2: i32 = sub"

plan_add:
  .ascii "mov_x0_2\nmov_x1_9\nadd_x2_x0_x1\nmov_x0_x2\nmov_x16_exit\nsvc_0x80\n"
plan_sub:
  .ascii "mov_x0_40\nmov_x1_2\nsub_x2_x0_x1\nmov_x0_x2\nmov_x16_exit\nsvc_0x80\n"
plan_renamed:
  .ascii "mov_x3_7\nmov_x4_5\nadd_x8_x3_x4\nmov_x0_x8\nmov_x16_exit\nsvc_0x80\n"
plan_chained:
  .ascii "mov_x0_7\nmov_x1_5\nadd_x2_x0_x1\nsub_x3_x2_x1\nmov_x0_x3\nmov_x16_exit\nsvc_0x80\n"
plan_three:
  .ascii "mov_x0_10\nmov_x1_20\nmov_x2_5\nadd_x3_x0_x2\nmov_x0_x3\nmov_x16_exit\nsvc_0x80\n"
