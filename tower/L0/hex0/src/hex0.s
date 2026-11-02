# rusteze hex0 — readable assembly REFERENCE for the hex expander (Linux x86-64).
#
# NOTE: this .s is no longer how the seed is built. The canonical seed is
# hand-encoded machine code in hex0/hex0.hand.hex (lowercase-only, self-hosting,
# no compiler in its trust path); hex0/hex0.bin is that hand-derived binary. This
# assembly is kept only as a human-readable description of the same logic (it
# additionally accepts A-F, which the minimal hand seed omits).
#
# Reads a hex source from stdin and writes the decoded bytes to stdout:
#   - pairs of hex digits become one byte (high nibble first);
#   - `#` starts a comment to end-of-line;
#   - every other character (whitespace, newlines, punctuation) is ignored.
#
# Registers: r12 = in cursor, r13 = in end, r14 = out cursor,
#            r15 = pending-nibble flag, r10 = held high nibble.

    .text
    .globl _start

_start:
    xor %eax, %eax                  # read(0, inbuf, 65536)
    xor %edi, %edi
    lea inbuf(%rip), %rsi
    mov $65536, %edx
    syscall
    test %rax, %rax
    js done                         # read error -> emit nothing
    lea inbuf(%rip), %r12
    lea (%r12,%rax), %r13
    lea outbuf(%rip), %r14
    xor %r15, %r15

loop:
    cmp %r13, %r12
    jae done
    movzbl (%r12), %eax
    inc %r12
    cmp $'#', %al
    je comment
    cmp $'0', %al
    jb loop                         # below '0' (ws, etc.) -> ignore
    cmp $'9', %al
    jbe d09
    cmp $'a', %al
    jb chk_AF
    cmp $'f', %al
    jbe daf
    jmp loop                        # above 'f' -> ignore
chk_AF:
    cmp $'A', %al
    jb loop
    cmp $'F', %al
    jbe dAF
    jmp loop                        # between 'F' and 'a' -> ignore
d09:
    sub $'0', %al
    jmp have
daf:
    sub $'a', %al
    add $10, %al
    jmp have
dAF:
    sub $'A', %al
    add $10, %al
have:
    movzbl %al, %eax                # eax = nibble 0..15
    test %r15, %r15
    jnz low_nibble
    mov %eax, %r10d                 # hold as high nibble
    mov $1, %r15
    jmp loop
low_nibble:
    shl $4, %r10d
    or %eax, %r10d
    movb %r10b, (%r14)              # emit one byte
    inc %r14
    xor %r15, %r15
    jmp loop

comment:
    cmp %r13, %r12
    jae done
    movzbl (%r12), %eax
    inc %r12
    cmp $10, %al                    # newline ends the comment
    jne comment
    jmp loop

done:
    lea outbuf(%rip), %rsi          # write(1, outbuf, out - outbuf)
    mov %r14, %rdx
    sub %rsi, %rdx
    mov $1, %eax
    mov $1, %edi
    syscall
    mov $60, %eax                   # exit(0)
    xor %edi, %edi
    syscall

    .bss
    .p2align 4
inbuf:
    .space 65536
outbuf:
    .space 65536
