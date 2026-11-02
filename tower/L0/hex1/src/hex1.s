# rusteze hex1 — hex0 plus single-char labels (Linux x86-64).
#
# The next assembler-ladder rung above hex0. Same hex/byte/comment rules, plus:
#   :X   define label X (one char) at the current output offset
#   }X   emit a 4-byte little-endian rel32 = offset(X) - (here + 4)
# so jump/call displacements are written symbolically instead of by hand. This is
# what makes larger programs (the rzir -> ELF emitter) hand-authorable.
#
# Two passes over the input: pass 1 fills a 256-entry label-offset table; pass 2
# emits bytes and resolves `}` references. Output is written once at the end.
#
# This .s is a readable reference only. The canonical hex1 is now DERIVED from
# hex1-seed.md (the constraints) per Doc 247: hand-authored in
# derived-hex1.labeled.hex, with jumps resolved by hex1 itself. The cc-built
# binary and its transcription are retired; cc is out of hex1's provenance.

    .text
    .globl _start

_start:
    xor %eax, %eax                  # read(0, inbuf, 65536)
    xor %edi, %edi
    lea inbuf(%rip), %rsi
    mov $65536, %edx
    syscall
    test %rax, %rax
    js exit0
    lea inbuf(%rip), %rbx           # rbx = input base
    lea (%rbx,%rax), %r13           # r13 = input end

    xor %r15d, %r15d                # pass 1 (build label table)
    call pass
    mov $1, %r15d                   # pass 2 (emit + resolve)
    call pass

    lea outbuf(%rip), %rsi          # write(1, outbuf, outlen)
    mov outlen(%rip), %rdx
    mov $1, %eax
    mov $1, %edi
    syscall
exit0:
    mov $60, %eax
    xor %edi, %edi
    syscall

# One pass over the input. r15 = 0 (record labels) or 1 (emit).
pass:
    mov %rbx, %rsi                  # rsi = input cursor
    xor %r12d, %r12d                # r12 = output offset (outpos)
    mov $0xff, %r14d                # r14 = pending high nibble (0xff = none)
p_loop:
    cmp %r13, %rsi
    jae p_done
    movzbl (%rsi), %eax
    inc %rsi
    cmp $'#', %al
    je p_comment
    cmp $':', %al
    je p_label
    cmp $'}', %al
    je p_ref
    # hex digit?
    cmp $'0', %al
    jb p_loop
    cmp $'9', %al
    jbe p_d09
    cmp $'a', %al
    jb p_loop
    cmp $'f', %al
    ja p_loop
    sub $0x57, %al                  # a..f -> 10..15
    jmp p_have
p_d09:
    sub $'0', %al
p_have:
    cmp $0xff, %r14d                # pending?
    jne p_low
    movzbl %al, %r14d               # hold high nibble
    jmp p_loop
p_low:
    shl $4, %r14b
    or %al, %r14b                   # byte = (hi<<4)|lo
    test %r15d, %r15d
    jz p_byte_adv                   # pass 1: just advance outpos
    lea outbuf(%rip), %rcx
    mov %r14b, (%rcx,%r12)          # pass 2: emit the byte
p_byte_adv:
    inc %r12
    mov $0xff, %r14d                # clear pending
    jmp p_loop

p_label:                            # :X  define label X at outpos
    cmp %r13, %rsi
    jae p_done
    movzbl (%rsi), %eax
    inc %rsi
    test %r15d, %r15d
    jnz p_loop                      # pass 2: nothing to do
    lea labeltab(%rip), %rcx
    mov %r12d, (%rcx,%rax,4)        # pass 1: labeltab[X] = outpos
    jmp p_loop

p_ref:                              # }X  emit rel32 to label X (4 bytes)
    cmp %r13, %rsi
    jae p_done
    movzbl (%rsi), %eax
    inc %rsi
    test %r15d, %r15d
    jz p_ref_adv                    # pass 1: just reserve 4 bytes
    lea labeltab(%rip), %rcx
    mov (%rcx,%rax,4), %edx         # target offset
    lea 4(%r12), %eax               # here + 4
    sub %eax, %edx                  # disp = target - (here+4)
    lea outbuf(%rip), %rcx
    mov %edx, (%rcx,%r12)           # emit 4 LE bytes
p_ref_adv:
    add $4, %r12
    jmp p_loop

p_comment:
    cmp %r13, %rsi
    jae p_done
    movzbl (%rsi), %eax
    inc %rsi
    cmp $10, %al
    jne p_comment
    jmp p_loop

p_done:
    mov %r12d, outlen(%rip)         # record output length (pass 2's value wins)
    ret

    .bss
    .p2align 4
inbuf:
    .space 65536
outbuf:
    .space 65536
labeltab:
    .space 1024                     # 256 dwords
outlen:
    .space 8
