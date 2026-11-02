import sys,re
# Robust test-code stripper for the LOWER sweep. Approximates "what rustc compiles with default features":
# remove #[test]/#[bench]/#[should_panic]/#[cfg(test)]/#[cfg(all(test,..))]/#[cfg_attr(test,..)] items, but NOT
# #[cfg(any(test,..))] (compiles under the other alternative). Item-extent scan is LEXER-AWARE (skips strings,
# char literals, line/block(nested) comments, raw strings) so a '{' / "}" / // { inside them never mis-counts
# braces -- the bug that left malformed (rustc-rejected) output and false "unmatched delimiter" sweep failures.
src=open(sys.argv[1],errors='ignore').read()
lines=src.split("\n")
def is_testattr(l):
    s=l.strip()
    if re.match(r'#!?\[\s*(test|bench|should_panic)\b', s): return True
    if re.match(r'#!?\[\s*cfg_attr\(\s*test\b', s): return True
    # cfg(...) requiring test: cfg(test) or cfg(all(test,..)); NOT cfg(any(test,..)) (other alt may compile)
    m=re.match(r'#!?\[\s*cfg\(\s*(.*)\)\s*\]\s*$', s)
    if m:
        inner=m.group(1).strip()
        if inner=='test': return True
        if inner.startswith('all(') and re.search(r'\btest\b', inner): return True
    return False
# lexer-aware forward scan from byte position p: return end position of the item (exclusive), counting braces
# while skipping strings/chars/comments/raw. Item ends at depth-0 '}' (if a '{' opened) or at ';' before any '{'.
def item_end(b, p):
    n=len(b); i=p; dep=0; opened=False
    while i<n:
        c=b[i]
        if c=='/' and i+1<n and b[i+1]=='/':
            j=b.find('\n', i); i=(n if j<0 else j); continue
        if c=='/' and i+1<n and b[i+1]=='*':
            d=1; i+=2
            while i<n and d>0:
                if b[i]=='/' and i+1<n and b[i+1]=='*': d+=1; i+=2
                elif b[i]=='*' and i+1<n and b[i+1]=='/': d-=1; i+=2
                else: i+=1
            continue
        if c in 'rbc':
            k=i
            if b[k] in 'bc' and k+1<n and b[k+1]=='r': k+=1
            if b[k]=='r' and k+1<n and (b[k+1]=='"' or b[k+1]=='#'):
                h=k+1
                while h<n and b[h]=='#': h+=1
                if h<n and b[h]=='"':
                    hashes=h-(k+1); j=h+1
                    while j<n:
                        if b[j]=='"':
                            m=j+1; cnt=0
                            while m<n and cnt<hashes and b[m]=='#': cnt+=1; m+=1
                            if cnt==hashes: j=m; break
                        j+=1
                    i=j; continue
        if c=="'":
            if i+1<n and b[i+1]=='\\':
                j=i+2
                while j<n and b[j]!="'": j+=1
                i=j+1; continue
            if i+2<n and b[i+2]=="'":
                i+=3; continue
            i+=1; continue
        if c=='"':
            j=i+1
            while j<n:
                if b[j]=='\\': j+=2; continue
                if b[j]=='"': break
                j+=1
            i=j+1; continue
        if c in '([': dep+=1; i+=1; continue
        if c in ')]': dep-=1; i+=1; continue
        if c=='{': dep+=1; opened=True; i+=1; continue
        if c=='}':
            dep-=1; i+=1
            if opened and dep==0: return i
            continue
        if c==';' and not opened and dep==0: return i+1
        i+=1
    return n
# build line-start byte offsets
offs=[0]
for ln in lines: offs.append(offs[-1]+len(ln)+1)
out=[]; i=0
while i<len(lines):
    if is_testattr(lines[i]):
        # pop preceding doc-comment (/// //!) and attribute (#[..]) lines already emitted -- they attach to the
        # item being stripped; leaving them dangles a "/// .. expected item after doc comment".
        while out and re.match(r'\s*(///|//!|#!?\[)', out[-1]): out.pop()
        while out and out[-1].strip()=='' and len(out)>=2 and re.match(r'\s*(///|//!)', out[-2]): out.pop()
        j=i
        while j<len(lines) and re.match(r'\s*#!?\[', lines[j]): j+=1  # skip stacked attrs
        if j<len(lines):
            # find the item's first non-whitespace byte at/after line j
            start=offs[j]
            while start<len(src) and src[start] in ' \t': start+=1
            e=item_end(src, start)
            endline=src[:e].count('\n')  # 0-based line index of last consumed char
            i=endline+1; continue
    out.append(lines[i]); i+=1
sys.stdout.write("\n".join(out)+("\n" if not src.endswith("\n") else "\n"))
