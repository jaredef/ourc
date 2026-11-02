.text
.globl _main
.p2align 2
_main:
  sub sp, sp, #2, lsl #12
  mov x19, sp
  add x27, x19, #1, lsl #12

  mov x16, #3
  mov x0, #0
  mov x1, x19
  mov x2, #4096
  svc #0x80
  add x20, x19, x0
  mov x26, x19

scan:
  cmp x26, x20
  b.hs exit_zero
  ldrb w1, [x26]
  cmp w1, #'v'
  b.eq maybe_assign
  cmp w1, #'r'
  b.eq maybe_ret
next_char:
  add x26, x26, #1
  b scan

maybe_assign:
  ldrb w2, [x26, #2]
  cmp w2, #':'
  b.ne next_char
  ldrb w22, [x26, #1]
  sub w22, w22, #'0'
  ldrb w2, [x26, #10]
  cmp w2, #'c'
  b.eq parse_const
  cmp w2, #'a'
  b.eq parse_add
  cmp w2, #'s'
  b.eq parse_sub
  b next_char

parse_const:
  add x24, x26, #16
  mov w25, #0
const_digits:
  ldrb w2, [x24]
  cmp w2, #'0'
  b.lo const_done
  cmp w2, #'9'
  b.hi const_done
  sub w2, w2, #'0'
  mov w3, #10
  madd w25, w25, w3, w2
  add x24, x24, #1
  b const_digits
const_done:
  bl emit_const
  b next_char

parse_add:
  ldrb w23, [x26, #15]
  sub w23, w23, #'0'
  ldrb w24, [x26, #18]
  sub w24, w24, #'0'
  bl emit_add
  b next_char

parse_sub:
  ldrb w23, [x26, #15]
  sub w23, w23, #'0'
  ldrb w24, [x26, #18]
  sub w24, w24, #'0'
  bl emit_sub
  b next_char

maybe_ret:
  ldrb w2, [x26, #1]
  cmp w2, #'e'
  b.ne next_char
  ldrb w2, [x26, #2]
  cmp w2, #'t'
  b.ne next_char
  ldrb w2, [x26, #4]
  cmp w2, #'v'
  b.ne next_char
  ldrb w22, [x26, #5]
  sub w22, w22, #'0'
  bl emit_ret_move
  adr x1, tok_exit
  mov x2, #13
  bl write_str
  adr x1, tok_svc
  mov x2, #9
  bl write_str
  b exit_zero

emit_const:
  cmp w22, #0
  b.eq const_x0
  cmp w22, #1
  b.eq const_x1
  cmp w22, #2
  b.eq const_x2
  cmp w22, #3
  b.eq const_x3
  cmp w22, #4
  b.eq const_x4
  cmp w22, #5
  b.eq const_x5
  cmp w22, #6
  b.eq const_x6
  ret
const_x0:
  cmp w25, #2
  b.eq emit_mov_x0_2
  cmp w25, #7
  b.eq emit_mov_x0_7
  cmp w25, #10
  b.eq emit_mov_x0_10
  cmp w25, #40
  b.eq emit_mov_x0_40
  ret
const_x1:
  cmp w25, #2
  b.eq emit_mov_x1_2
  cmp w25, #5
  b.eq emit_mov_x1_5
  cmp w25, #9
  b.eq emit_mov_x1_9
  cmp w25, #20
  b.eq emit_mov_x1_20
  ret
const_x2:
  cmp w25, #5
  b.eq emit_mov_x2_5
  ret
const_x3:
  cmp w25, #7
  b.eq emit_mov_x3_7
  ret
const_x4:
  cmp w25, #5
  b.eq emit_mov_x4_5
  ret
const_x5:
  cmp w25, #3
  b.eq emit_mov_x5_3
  ret
const_x6:
  cmp w25, #4
  b.eq emit_mov_x6_4
  ret

emit_add:
  cmp w22, #2
  b.eq add_x2
  cmp w22, #3
  b.eq add_x3
  cmp w22, #7
  b.eq add_x7
  cmp w22, #8
  b.eq add_x8
  ret
add_x2:
  cmp w23, #0
  b.ne ret_only
  cmp w24, #1
  b.eq emit_add_x2_x0_x1
  ret
add_x3:
  cmp w23, #0
  b.ne ret_only
  cmp w24, #2
  b.eq emit_add_x3_x0_x2
  ret
add_x8:
  cmp w23, #3
  b.ne ret_only
  cmp w24, #4
  b.eq emit_add_x8_x3_x4
  ret
add_x7:
  cmp w23, #5
  b.ne ret_only
  cmp w24, #6
  b.eq emit_add_x7_x5_x6
  ret

emit_sub:
  cmp w22, #2
  b.eq sub_x2
  cmp w22, #3
  b.eq sub_x3
  ret
sub_x2:
  cmp w23, #0
  b.ne ret_only
  cmp w24, #1
  b.eq emit_sub_x2_x0_x1
  ret
sub_x3:
  cmp w23, #2
  b.ne ret_only
  cmp w24, #1
  b.eq emit_sub_x3_x2_x1
  ret

emit_ret_move:
  cmp w22, #2
  b.eq emit_mov_x0_x2
  cmp w22, #3
  b.eq emit_mov_x0_x3
  cmp w22, #7
  b.eq emit_mov_x0_x7
  cmp w22, #8
  b.eq emit_mov_x0_x8
ret_only:
  ret

emit_mov_x0_2:
  adr x1, tok_mov_x0_2
  mov x2, #9
  b write_str
emit_mov_x0_7:
  adr x1, tok_mov_x0_7
  mov x2, #9
  b write_str
emit_mov_x0_10:
  adr x1, tok_mov_x0_10
  mov x2, #10
  b write_str
emit_mov_x0_40:
  adr x1, tok_mov_x0_40
  mov x2, #10
  b write_str
emit_mov_x1_2:
  adr x1, tok_mov_x1_2
  mov x2, #9
  b write_str
emit_mov_x1_5:
  adr x1, tok_mov_x1_5
  mov x2, #9
  b write_str
emit_mov_x1_9:
  adr x1, tok_mov_x1_9
  mov x2, #9
  b write_str
emit_mov_x1_20:
  adr x1, tok_mov_x1_20
  mov x2, #10
  b write_str
emit_mov_x2_5:
  adr x1, tok_mov_x2_5
  mov x2, #9
  b write_str
emit_mov_x3_7:
  adr x1, tok_mov_x3_7
  mov x2, #9
  b write_str
emit_mov_x4_5:
  adr x1, tok_mov_x4_5
  mov x2, #9
  b write_str
emit_mov_x5_3:
  adr x1, tok_mov_x5_3
  mov x2, #9
  b write_str
emit_mov_x6_4:
  adr x1, tok_mov_x6_4
  mov x2, #9
  b write_str
emit_add_x2_x0_x1:
  adr x1, tok_add_x2_x0_x1
  mov x2, #13
  b write_str
emit_add_x3_x0_x2:
  adr x1, tok_add_x3_x0_x2
  mov x2, #13
  b write_str
emit_add_x7_x5_x6:
  adr x1, tok_add_x7_x5_x6
  mov x2, #13
  b write_str
emit_add_x8_x3_x4:
  adr x1, tok_add_x8_x3_x4
  mov x2, #13
  b write_str
emit_sub_x2_x0_x1:
  adr x1, tok_sub_x2_x0_x1
  mov x2, #13
  b write_str
emit_sub_x3_x2_x1:
  adr x1, tok_sub_x3_x2_x1
  mov x2, #13
  b write_str
emit_mov_x0_x2:
  adr x1, tok_mov_x0_x2
  mov x2, #10
  b write_str
emit_mov_x0_x3:
  adr x1, tok_mov_x0_x3
  mov x2, #10
  b write_str
emit_mov_x0_x7:
  adr x1, tok_mov_x0_x7
  mov x2, #10
  b write_str
emit_mov_x0_x8:
  adr x1, tok_mov_x0_x8
  mov x2, #10
  b write_str

write_str:
  str x20, [x27]
  str x26, [x27, #8]
  mov x16, #4
  mov x0, #1
  svc #0x80
  ldr x20, [x27]
  ldr x26, [x27, #8]
  ret

exit_zero:
  mov x16, #1
  mov x0, #0
  svc #0x80

tok_mov_x0_2: .ascii "mov_x0_2\n"
tok_mov_x0_7: .ascii "mov_x0_7\n"
tok_mov_x0_10: .ascii "mov_x0_10\n"
tok_mov_x0_40: .ascii "mov_x0_40\n"
tok_mov_x1_2: .ascii "mov_x1_2\n"
tok_mov_x1_5: .ascii "mov_x1_5\n"
tok_mov_x1_9: .ascii "mov_x1_9\n"
tok_mov_x1_20: .ascii "mov_x1_20\n"
tok_mov_x2_5: .ascii "mov_x2_5\n"
tok_mov_x3_7: .ascii "mov_x3_7\n"
tok_mov_x4_5: .ascii "mov_x4_5\n"
tok_mov_x5_3: .ascii "mov_x5_3\n"
tok_mov_x6_4: .ascii "mov_x6_4\n"
tok_add_x2_x0_x1: .ascii "add_x2_x0_x1\n"
tok_add_x3_x0_x2: .ascii "add_x3_x0_x2\n"
tok_add_x7_x5_x6: .ascii "add_x7_x5_x6\n"
tok_add_x8_x3_x4: .ascii "add_x8_x3_x4\n"
tok_sub_x2_x0_x1: .ascii "sub_x2_x0_x1\n"
tok_sub_x3_x2_x1: .ascii "sub_x3_x2_x1\n"
tok_mov_x0_x2: .ascii "mov_x0_x2\n"
tok_mov_x0_x3: .ascii "mov_x0_x3\n"
tok_mov_x0_x7: .ascii "mov_x0_x7\n"
tok_mov_x0_x8: .ascii "mov_x0_x8\n"
tok_exit: .ascii "mov_x16_exit\n"
tok_svc: .ascii "svc_0x80\n"
