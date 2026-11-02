.text
.globl _main
.align 2

# arm64/Darwin oracle for the future macho-hex1 body.
#
# This is not the trusted seed. It is an authoring oracle that fixes the
# behavioral shape before hand-encoding:
#   hex0-compatible pair decoding
#   :X label definitions
#   @X -> arm64 b X instruction
#   !X -> arm64 bl X instruction
#
# Stack layout, from low to high:
#   input  0x00000..0x0ffff
#   output 0x10000..0x1ffff
#   labels 0x20000..0x203ff  (256 x u32 offsets)

_main:
  mov x9, #0x21
  lsl x9, x9, #12
  sub sp, sp, x9
  mov x10, sp                  // input base
  add x12, x10, #0x10, lsl #12 // output base
  add x13, x10, #0x20, lsl #12 // label table

  mov x16, #3                  // read
  mov x0, #0
  mov x1, x10
  mov x2, #0x10000
  svc #0x80

  add x11, x10, x0             // input end
  mov w15, #0                  // pass 0: define labels / measure
  bl pass
  mov w15, #1                  // pass 1: emit
  bl pass

  mov x16, #4                  // write
  mov x0, #1
  mov x1, x12
  mov x2, x20                  // output length
  svc #0x80

  mov x16, #1                  // exit
  mov x0, #0
  svc #0x80

pass:
  mov x14, x10                 // cursor
  mov x20, #0                  // output offset
  mov x19, #-1                 // pending nibble sentinel

loop:
  cmp x14, x11
  b.hs done
  ldrb w21, [x14], #1

  cmp w21, #'#'
  b.eq comment
  cmp w21, #':'
  b.eq define_label
  cmp w21, #'@'
  b.eq branch_b
  cmp w21, #'!'
  b.eq branch_bl

  cmp w21, #'0'
  b.lo loop
  cmp w21, #'9'
  b.ls digit
  cmp w21, #'a'
  b.lo loop
  cmp w21, #'f'
  b.hi loop
  sub w21, w21, #87
  b have_nibble

digit:
  sub w21, w21, #'0'

have_nibble:
  cmp x19, #-1
  b.ne low_nibble
  mov x19, x21
  b loop

low_nibble:
  lsl x19, x19, #4
  orr x19, x19, x21
  cbz w15, byte_advance
  strb w19, [x12, x20]

byte_advance:
  add x20, x20, #1
  mov x19, #-1
  b loop

define_label:
  ldrb w21, [x14], #1
  cbnz w15, loop
  str w20, [x13, x21, lsl #2]
  b loop

branch_b:
  mov w22, #0x14000000
  b branch_ref

branch_bl:
  mov w22, #0x94000000

branch_ref:
  ldrb w21, [x14], #1
  cbz w15, ref_advance
  ldr w23, [x13, x21, lsl #2]  // target offset
  sub w23, w23, w20            // byte delta from instruction start
  asr w23, w23, #2             // imm26
  and w23, w23, #0x03ffffff
  orr w23, w23, w22
  str w23, [x12, x20]

ref_advance:
  add x20, x20, #4
  b loop

comment:
  cmp x14, x11
  b.hs done
  ldrb w21, [x14], #1
  cmp w21, #10
  b.ne comment
  b loop

done:
  ret
