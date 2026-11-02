.text
.globl _main
.align 2

# arm64/Darwin oracle for macho-M0 slice 1.
#
# Slice 1 implements tokenized hex only:
#   whitespace separates tokens;
#   # begins a comment through newline;
#   even-length hex tokens emit bytes;
#   odd-length or non-hex bare tokens exit 3.

_main:
  mov x9, #0x20
  lsl x9, x9, #12
  sub sp, sp, x9
  mov x10, sp                  // input base
  add x12, x10, #0x10, lsl #12 // output base

  mov x16, #3                  // read
  mov x0, #0
  mov x1, x10
  mov x2, #0x10000
  svc #0x80

  add x11, x10, x0             // input end
  mov x14, x10                 // cursor
  mov x20, #0                  // output offset

scan:
  cmp x14, x11
  b.hs write_output
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
  bl emit_hex_token
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
  strb w25, [x12, x20]
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

write_output:
  mov x16, #4                  // write
  mov x0, #1
  mov x1, x12
  mov x2, x20
  svc #0x80

  mov x16, #1                  // exit 0
  mov x0, #0
  svc #0x80

error:
  mov x16, #1                  // exit 3
  mov x0, #3
  svc #0x80

