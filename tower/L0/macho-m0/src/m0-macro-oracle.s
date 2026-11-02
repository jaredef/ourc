.text
.globl _main
.align 2

# arm64/Darwin oracle for macho-M0 slice 3.
#
# Slice 3 implements:
#   tokenized hex;
#   multi-character labels;
#   @NAME / !NAME arm64 branch references;
#   ?WORD:NAME arm64 imm19 word-patching references;
#   &NAME output offsets;
#   =NAME preferred __TEXT.__text addresses.
#   %NAME HEX macro definitions;
#   bare NAME macro expansion.

_main:
  mov x9, #0x30
  lsl x9, x9, #12
  sub sp, sp, x9
  mov x10, sp                  // input base
  add x12, x10, #0x10, lsl #12 // output base
  add x13, x10, #0x20, lsl #12 // labels base
  add x17, x10, #0x28, lsl #12 // macros base

  mov x16, #3                  // read
  mov x0, #0
  mov x1, x10
  mov x2, #0x10000
  svc #0x80

  add x11, x10, x0             // input end
  mov x27, #0                  // label count
  mov x6, #0                   // macro count
  mov w15, #0                  // pass 0: labels / measure
  bl pass
  mov w15, #1                  // pass 1: emit / resolve
  bl pass

  mov x16, #4                  // write
  mov x0, #1
  mov x1, x12
  mov x2, x20
  svc #0x80

  mov x16, #1                  // exit 0
  mov x0, #0
  svc #0x80

pass:
  stp x29, x30, [sp, #-16]!
  mov x29, sp
  mov x14, x10                 // cursor
  mov x20, #0                  // output offset

scan:
  cmp x14, x11
  b.hs done
  ldrb w21, [x14]

  cmp w21, #' '
  b.eq skip_one
  cmp w21, #9
  b.eq skip_one
  cmp w21, #10
  b.eq skip_one
  cmp w21, #'#'
  b.eq comment

  mov x22, x14                 // token start
  bl scan_token
  mov x23, x14
  sub x23, x23, x22            // token length
  ldrb w21, [x22]              // token head

  cmp w21, #':'
  b.eq label_def
  cmp w21, #'@'
  b.eq branch_b
  cmp w21, #'!'
  b.eq branch_bl
  cmp w21, #'&'
  b.eq offset_ref
  cmp w21, #'='
  b.eq address_ref
  cmp w21, #'?'
  b.eq imm19_ref
  cmp w21, #'%'
  b.eq macro_def

  bl macro_or_hex
  b scan

skip_one:
  add x14, x14, #1
  b scan

comment:
  add x14, x14, #1
comment_loop:
  cmp x14, x11
  b.hs scan
  ldrb w21, [x14], #1
  cmp w21, #10
  b.ne comment_loop
  b scan

scan_token:
  cmp x14, x11
  b.hs scan_token_done
  ldrb w21, [x14]
  cmp w21, #' '
  b.eq scan_token_done
  cmp w21, #9
  b.eq scan_token_done
  cmp w21, #10
  b.eq scan_token_done
  add x14, x14, #1
  b scan_token
scan_token_done:
  ret

label_def:
  cbnz w15, scan
  add x0, x22, #1              // name ptr
  sub x1, x23, #1              // name len
  cbz x1, error
  lsl x24, x27, #4             // 16-byte label entry
  add x24, x13, x24
  str x0, [x24]
  str w1, [x24, #8]
  str w20, [x24, #12]
  add x27, x27, #1
  b scan

branch_b:
  mov w5, #0x14000000
  b branch_ref

branch_bl:
  mov w5, #0x94000000

branch_ref:
  cbz w15, ref4_advance
  add x0, x22, #1
  sub x1, x23, #1
  cbz x1, error
  bl lookup_label              // w0 = target offset
  sub w0, w0, w20
  asr w0, w0, #2
  and w0, w0, #0x03ffffff
  orr w0, w0, w5
  str w0, [x12, x20]

ref4_advance:
  add x20, x20, #4
  b scan

offset_ref:
  cbz w15, ref4_advance
  add x0, x22, #1
  sub x1, x23, #1
  cbz x1, error
  bl lookup_label
  str w0, [x12, x20]
  b ref4_advance

address_ref:
  cbz w15, ref8_advance
  add x0, x22, #1
  sub x1, x23, #1
  cbz x1, error
  bl lookup_label
  mov x24, #0x2b0
  movk x24, #0x0001, lsl #32
  add x0, x0, x24
  str x0, [x12, x20]

ref8_advance:
  add x20, x20, #8
  b scan

imm19_ref:
  cmp x23, #11                 // ? + 8 hex + : + at least 1-char name
  b.lo error
  add x24, x22, #9
  ldrb w0, [x24]
  cmp w0, #':'
  b.ne error

  add x24, x22, #1             // word hex cursor
  mov x25, #0                  // digit index
  mov w5, #0                   // base instruction word
imm19_word_loop:
  cmp x25, #8
  b.hs imm19_word_done
  ldrb w0, [x24, x25]
  bl hexval
  lsl w5, w5, #4
  orr w5, w5, w0
  add x25, x25, #1
  b imm19_word_loop
imm19_word_done:
  cbz w15, ref4_advance
  add x0, x22, #10             // label ptr after ?WORD:
  sub x1, x23, #10             // label len
  cbz x1, error
  bl lookup_label              // w0 = target offset
  sub w0, w0, w20
  asr w0, w0, #2
  and w0, w0, #0x0007ffff
  lsl w0, w0, #5
  mov w3, #0xffe0
  movk w3, #0x00ff, lsl #16
  bic w5, w5, w3               // clear imm19 bits 5..23
  orr w0, w0, w5
  str w0, [x12, x20]
  b ref4_advance

macro_def:
  add x0, x22, #1              // macro name ptr
  sub x1, x23, #1              // macro name len
  cbz x1, error
  bl skip_whitespace
  mov x2, x14                  // body token ptr
  bl scan_token
  mov x3, x14
  sub x3, x3, x2               // body token hex length
  cbz x3, error
  tst x3, #1
  b.ne error
  cbnz w15, scan
  lsl x24, x6, #5              // 32-byte macro entry
  add x24, x17, x24
  str x0, [x24]                // name ptr
  str w1, [x24, #8]            // name len
  str x2, [x24, #16]           // body ptr
  str w3, [x24, #24]           // body hex len
  lsr x4, x3, #1
  str w4, [x24, #28]           // body byte len
  add x6, x6, #1
  b scan

skip_whitespace:
  cmp x14, x11
  b.hs error
  ldrb w21, [x14]
  cmp w21, #' '
  b.eq skip_ws_one
  cmp w21, #9
  b.eq skip_ws_one
  cmp w21, #10
  b.eq skip_ws_one
  ret
skip_ws_one:
  add x14, x14, #1
  b skip_whitespace

macro_or_hex:
  mov x0, x22
  mov x1, x23
  bl lookup_macro
  cbz x7, emit_hex_token       // x7=0 means not found
  mov x22, x2                  // macro body ptr
  mov x23, x3                  // macro body hex len
  b emit_hex_token

lookup_macro:
  mov x7, #0
  mov x24, #0
lookup_macro_loop:
  cmp x24, x6
  b.hs lookup_macro_done
  lsl x25, x24, #5
  add x25, x17, x25
  ldr w26, [x25, #8]
  cmp w26, w1
  b.ne lookup_macro_next
  ldr x2, [x25]
  mov x28, #0
compare_macro_loop:
  cmp x28, x1
  b.hs lookup_macro_match
  ldrb w3, [x2, x28]
  ldrb w4, [x0, x28]
  cmp w3, w4
  b.ne lookup_macro_next
  add x28, x28, #1
  b compare_macro_loop
lookup_macro_match:
  ldr x2, [x25, #16]
  ldr w3, [x25, #24]
  mov x7, #1
  ret
lookup_macro_next:
  add x24, x24, #1
  b lookup_macro_loop
lookup_macro_done:
  ret

lookup_label:
  mov x24, #0                  // i
lookup_loop:
  cmp x24, x27
  b.hs error
  lsl x25, x24, #4
  add x25, x13, x25
  ldr w26, [x25, #8]           // entry len
  cmp w26, w1
  b.ne lookup_next
  ldr x2, [x25]                // entry ptr
  mov x28, #0                  // k
compare_loop:
  cmp x28, x1
  b.hs lookup_match
  ldrb w3, [x2, x28]
  ldrb w4, [x0, x28]
  cmp w3, w4
  b.ne lookup_next
  add x28, x28, #1
  b compare_loop
lookup_match:
  ldr w0, [x25, #12]
  ret
lookup_next:
  add x24, x24, #1
  b lookup_loop

emit_hex_token:
  tst x23, #1
  b.ne error
  mov x24, #0                  // token index
emit_loop:
  cmp x24, x23
  b.hs scan
  ldrb w0, [x22, x24]
  bl hexval
  lsl w25, w0, #4
  add x24, x24, #1
  ldrb w0, [x22, x24]
  bl hexval
  orr w25, w25, w0
  cbz w15, emit_advance
  strb w25, [x12, x20]
emit_advance:
  add x20, x20, #1
  add x24, x24, #1
  b emit_loop

hexval:
  cmp w0, #'0'
  b.lo error
  cmp w0, #'9'
  b.ls hex_digit
  cmp w0, #'A'
  b.lo error
  cmp w0, #'F'
  b.ls hex_upper
  cmp w0, #'a'
  b.lo error
  cmp w0, #'f'
  b.ls hex_lower
  b error

hex_digit:
  sub w0, w0, #'0'
  ret

hex_upper:
  sub w0, w0, #55
  ret

hex_lower:
  sub w0, w0, #87
  ret

done:
  ldp x29, x30, [sp], #16
  ret

error:
  mov x16, #1                  // exit 3
  mov x0, #3
  svc #0x80
