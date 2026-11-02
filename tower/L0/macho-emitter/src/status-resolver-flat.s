.text
.globl _main
.p2align 2
_main:
  sub sp, sp, #2, lsl #12
  mov x19, sp
  add x21, x19, #1, lsl #12

  mov x16, #3
  mov x0, #0
  mov x1, x19
  mov x2, #4096
  svc #0x80
  add x20, x19, x0

  mov x0, x19

scan:
  cmp x0, x20
  b.hs done_zero
  ldrb w1, [x0]
  cmp w1, #'v'
  b.eq maybe_assign
  cmp w1, #'r'
  b.eq maybe_ret
next_char:
  add x0, x0, #1
  b scan

maybe_assign:
  ldrb w2, [x0, #2]
  cmp w2, #':'
  b.ne next_char
  ldrb w22, [x0, #1]
  sub w22, w22, #'0'
  add x23, x0, #10
  ldrb w2, [x23]
  cmp w2, #'c'
  b.eq parse_const
  cmp w2, #'a'
  b.eq parse_add
  cmp w2, #'s'
  b.eq parse_sub
  b next_char

parse_const:
  add x24, x0, #16
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
  strb w25, [x21, x22]
  b next_char

parse_add:
  ldrb w2, [x0, #15]
  sub w2, w2, #'0'
  ldrb w3, [x0, #18]
  sub w3, w3, #'0'
  ldrb w4, [x21, x2]
  ldrb w5, [x21, x3]
  add w4, w4, w5
  strb w4, [x21, x22]
  b next_char

parse_sub:
  ldrb w2, [x0, #15]
  sub w2, w2, #'0'
  ldrb w3, [x0, #18]
  sub w3, w3, #'0'
  ldrb w4, [x21, x2]
  ldrb w5, [x21, x3]
  sub w4, w4, w5
  strb w4, [x21, x22]
  b next_char

maybe_ret:
  ldrb w2, [x0, #1]
  cmp w2, #'e'
  b.ne next_char
  ldrb w2, [x0, #2]
  cmp w2, #'t'
  b.ne next_char
  ldrb w2, [x0, #3]
  cmp w2, #' '
  b.ne next_char
  ldrb w2, [x0, #4]
  cmp w2, #'v'
  b.ne next_char
  ldrb w2, [x0, #5]
  sub w2, w2, #'0'
  ldrb w22, [x21, x2]
  b write_status

done_zero:
  mov w22, #0

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
