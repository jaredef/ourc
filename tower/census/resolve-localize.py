#!/usr/bin/env python3
# resolve-localize.py <SEM0> <spliced_file> --cwd DIR
# Localize the construct that makes SEM0 //@S3RESOLVE refuse (rc 101) on a spliced crate. SEM0 refuses SILENTLY
# (no diagnostic), so we delta-debug: repeatedly blank the BODY of a brace-block (any nesting depth) while the
# refusal (rc==101) persists. Brace/paren/bracket depth is tracked with string/char/comment skipping so we only
# cut at real delimiters. Prints the minimal still-refusing file's remaining non-blank lines = the meet.
import sys, subprocess, os
FE, PATH = sys.argv[1], sys.argv[2]
CWD = sys.argv[sys.argv.index('--cwd')+1] if '--cwd' in sys.argv else None
def refuses(text):
    p = subprocess.run([FE], input="//@S3RESOLVE\n"+text, capture_output=True, text=True, cwd=CWD)
    return p.returncode == 101
# tokenize brace pairs with string/comment skipping
def brace_pairs(s):
    pairs=[]; stack=[]; i=0; n=len(s); inl=False; inb=False; instr=None
    while i<n:
        c=s[i]; d=s[i+1] if i+1<n else ''
        if inl:
            if c=='\n': inl=False
            i+=1; continue
        if inb:
            if c=='*' and d=='/': inb=False; i+=2; continue
            i+=1; continue
        if instr:
            if c=='\\': i+=2; continue
            if c==instr: instr=None
            i+=1; continue
        if c=='/' and d=='/': inl=True; i+=2; continue
        if c=='/' and d=='*': inb=True; i+=2; continue
        if c=='"': instr='"'; i+=1; continue
        if c=="'":
            # char literal or lifetime; treat '{' '}' chars: skip a simple char literal
            if i+2<n and s[i+2]=="'" or (i+1<n and s[i+1]=='\\'):
                instr="'"; i+=1; continue
            i+=1; continue
        if c=='{': stack.append(i)
        elif c=='}':
            if stack: pairs.append((stack.pop(), i))
        i+=1
    return pairs
text=open(PATH).read()
if not refuses(text):
    print("POSITIVE CONTROL FAILED: file does not refuse (rc!=101)"); sys.exit(1)
changed=True
while changed:
    changed=False
    pairs=sorted(brace_pairs(text), key=lambda p:-(p[1]-p[0]))  # largest bodies first
    for a,b in pairs:
        if b-a<=1: continue
        cand=text[:a+1]+"\n"+text[b:]   # blank the body
        if refuses(cand):
            text=cand; changed=True; break
lines=[l for l in text.split('\n') if l.strip()]
print(f"=== minimal refusing residue ({len(lines)} non-blank lines) ===")
print('\n'.join(lines))
