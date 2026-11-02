#!/usr/bin/env sh
# c-to-rzir.m0 -- the derived C front-end.
#   RUNG 1: int main() { return <additive int expr>; }
#   RUNG 2: local int vars, declarations, assignments, variable refs, ; statements
set -eu
cd "$(dirname "$0")"
M0=../../L0/m0/m0.bin
CF=../../L0/emitter/rzir-to-elf-cf.m0
mkdir -p build; chmod +x "$M0"
"$M0" < c-to-rzir.m0 > build/cc.bin; chmod +x build/cc.bin
"$M0" < "$CF" > build/cf.bin; chmod +x build/cf.bin
chk() { printf 'int main() { %s }\n' "$1" | ./build/cc.bin | ./build/cf.bin > build/o; chmod +x build/o
  set +e; ./build/o; r="$?"; set -e; [ "$r" = "$2" ] || { echo "C: {$1} -> $r != $2" >&2; exit 1; }; }
# rung 1: additive expressions
chk "return 1+2;" 3 ; chk "return 5;" 5 ; chk "return 1+2+3;" 6 ; chk "return 100-50+5;" 55
# rung 2: locals, decls, assignment, variable refs, statement sequences
chk "int x = 5; int y = x + 1; return y;" 6
chk "int x = 10; return x;" 10
chk "int x = 3; int y = 4; return x + y;" 7
chk "int x = 1; x = x + 9; return x;" 10
chk "int a = 2; int b = 3; int c = a + b + 5; return c;" 10
# rung 3: precedence (* / bind tighter than + -) and parentheses
chk "return 2 + 3 * 4;" 14 ; chk "return (2 + 3) * 4;" 20 ; chk "return 2 * 3 + 4;" 10
chk "return 20 / 4 - 1;" 4 ; chk "return 2 * (3 + 4) - 5;" 9 ; chk "return ((1 + 2) * (3 + 1));" 12
chk "return 1 + 2 * 3 + 4 * 5;" 27 ; chk "return 100 - 2 * 3 * 4;" 76
# rung 4: comparisons, if, while, stable slots, nested control flow
chk "return 3 < 5;" 1 ; chk "return 5 == 5;" 1 ; chk "return 4 != 5;" 1 ; chk "return 6 >= 6;" 1
chk "int x = 7; if (5 < 3) { x = 9; } return x;" 7 ; chk "int x = 2; if (3 < 5) { x = 9; } return x;" 9
chk "int s = 0; int i = 0; while (i < 10) { s = s + i; i = i + 1; } return s;" 45
chk "int n = 5; int f = 1; while (n > 1) { f = f * n; n = n - 1; } return f;" 120
chk "int s = 0; int i = 0; while (i < 6) { if (i > 2) { s = s + i; } i = i + 1; } return s;" 12
# rung 5: functions, parameters, single-arg calls, recursion
P() { printf '%s\n' "$1" | ./build/cc.bin | ./build/cf.bin > build/o; chmod +x build/o
  set +e; ./build/o; r="$?"; set -e; [ "$r" = "$2" ] || { echo "C: [$1] -> $r != $2" >&2; exit 1; }; }
P "int main() { return dbl(5); } int dbl(int x) { return x + x; }" 10
P "int main() { return inc(41); } int inc(int x) { return x + 1; }" 42
P "int main() { return fact(5); } int fact(int n) { if (n < 2) { return 1; } return n * fact(n - 1); }" 120
P "int main() { return f() + g(); } int f() { return 3; } int g() { return 4; }" 7
P "int main() { return add(3, 4); } int add(int a, int b) { return a + b; }" 7
P "int main() { return add(3, 4, 5); } int add(int a, int b, int c) { return a + b + c; }" 12
P "int main() { return sub(10, 3); } int sub(int a, int b) { return a - b; }" 7
# rung 6: pointers -- &x (ref), *p (load), *p = v (store)
chk "int x = 5; int *p = &x; return *p;" 5
chk "int x = 5; int *p = &x; *p = 9; return x;" 9
chk "int x = 3; int *p = &x; *p = *p + 4; return x;" 7
chk "int x=1; int y=2; int *p=&x; int *q=&y; *p = *q + 10; return x;" 12
# rung 6 arrays: int a[N], a[i] get/put, with loops + multi-digit (>10) slots
chk "int a[3]; a[0]=7; a[1]=8; a[2]=9; return a[0]+a[2];" 16
chk "int a[5]; int i=0; while(i<5){a[i]=i;i=i+1;} return a[4];" 4
chk "int a[5]; int i=0; int s=0; while(i<5){a[i]=i;i=i+1;} i=0; while(i<5){s=s+a[i];i=i+1;} return s;" 10
chk "int a=1;int b=1;int c=1;int d=1;int e=1;int f=1;int g=1;int h=1;int j=1;int k=1;int l=1;int m=1; return a+b+c+d+e+f+g+h+j+k+l+m;" 12
# char: literals, the char keyword (i32 synonym), char params, char arrays
chk "return 'A';" 65
chk "return 'z';" 122
chk "char c = 'h'; return c;" 104
chk "char b[3]; b[0] = 'h'; b[1] = 'i'; return b[0] + b[1];" 209
P "int main() { return up('a'); } int up(char c) { return c - 32; }" 65
# R1 (c-parity): multi-character identifiers (string->slot symbol table)
chk "int count = 5; int total = count + 1; return total;" 6
chk "int sum = 0; int idx = 0; while (idx < 5) { sum = sum + idx; idx = idx + 1; } return sum;" 10
P "int main() { return square(6); } int square(int num) { return num * num; }" 36
P "int main() { return addup(3, 4, 5); } int addup(int aa, int bb, int cc) { return aa + bb + cc; }" 12
chk "int val = 7; int point = 0; point = &val; return *point;" 7
# R2 (c-parity): comments (// and /* */), division preserved
R() { printf '%b\n' "$1" | ./build/cc.bin | ./build/cf.bin > build/o; chmod +x build/o; set +e; ./build/o; r="$?"; set -e; [ "$r" = "$2" ] || { echo "C: $1 -> $r != $2" >&2; exit 1; }; }
R "int main() { /* b */ return 5; }" 5
R "int main() { return 7; // line\n}" 7
R "int main() { int n = 3; /* m\nl */ return n + 4; }" 7
R "int main() { return 20 / 4; }" 5
R "int main() { int x=0; if (1) {x=5;} else {x=9;} return x; }" 5
R "int main() { int x=0; if (0) {x=5;} else {x=9;} return x; }" 9
R "int main() { int n=2; int r=0; if (n==1){r=10;} else if (n==2){r=20;} else {r=30;} return r; }" 20
R "int main() { int s=0; for (int i=0;i<5;i=i+1) {s=s+i;} return s; }" 10
R "int main() { int p=1; for (int i=1;i<=5;i=i+1) {p=p*i;} return p; }" 120
R "int main() { int c=0; for (int n=1;n<=4;n=n+1) { if (n==2){c=c+10;} else {c=c+1;} } return c; }" 13
# R4 (c-parity): % (modulo), unary minus, logical not
R "int main() { return 17 % 5; }" 2
R "int main() { return 2 + 3 * 4 % 5; }" 4
R "int main() { return -5 + 8; }" 3
R "int main() { return !0; }" 1
# R4: short-circuit && || (rhs skipped -> the div-by-zero is never evaluated)
R "int main() { return 1 && 0; }" 0
# R4: statement ++ -- += -= (compound assignment); front-end now > 256 labels
R "int main() { int i=0; i++; return i; }" 1
R "int main() { int i=5; i--; return i; }" 4
R "int main() { int x=5; x += 3; return x; }" 8
R "int main() { int x=5; x *= 3; return x; }" 15
R "int main() { int x=20; x /= 4; return x; }" 5
R "int main() { int s=0; int i=0; while (i<5) { s += i; i++; } return s; }" 10
R "int main() { int s=0; for (int i=0;i<5;i++) s=s+i; return s; }" 10
R "int main() { int p=1; for (int i=1;i<=5;i++) p=p*i; return p; }" 120
R "int main() { int s=0; for (int i=0;i<10;i+=2) s=s+i; return s; }" 20
R "int main() { int c=0; for (int i=10;i>0;i--) c++; return c; }" 10
R "int main() { return 0 || 7; }" 1
R "int main() { int n=0; if (n != 0 && 5 / n > 0) { return 99; } return 7; }" 7
R "int main() { int a=3; int b=4; if (a>0 && b>0) { return a+b; } return 0; }" 7
# R4 cont: bitwise & | ^
R "int main() { return 6 & 3; }" 2
R "int main() { return sizeof(int) + sizeof(char); }" 5
R "int main() { return sizeof(int*); }" 8
R "int main() { int v=7; int p=0; p=&v; return *(int*)p; }" 7
R "int main() { unsigned int x = 5; return x; }" 5
R "int main() { unsigned long total = 42; return total; }" 42
R "int main() { long sum = 0; for (int i = 0; i < 5; i++) sum += i; return sum; }" 10
R "int main() { int len = 9; int used = 3; return len + used - 5; }" 7
R "int main() { int item=9; return (item); }" 9
R "int main() { return (int)10 + (int)5; }" 15
R "int main() { return 5 | 2; }" 7
R "int main() { return 6 ^ 3; }" 5
R "int main() { int n=13; int cnt=0; for (int i=0;i<4;i=i+1){ if (n&1) cnt=cnt+1; n=n/2; } return cnt; }" 3
# R4 cont: ternary ?: (cf sel)
R "int main() { return 1 ? 5 : 9; }" 5
R "int main() { int n=7; return n>5 ? 100 : 1; }" 100
R "int main() { int a=3; int b=8; return a>b ? a : b; }" 8
R "int main() { int n=2; return n==1 ? 10 : n==2 ? 20 : 30; }" 20
R "int main() { int c=0; for (int n=1; n<=15; n=n+1) { if (n%15==0){c=c+100;} else if (n%3==0){c=c+10;} else if (n%5==0){c=c+5;} else {c=c+1;} } return c; }" 158
# unbraced single-statement bodies + the literal gauntlet fizzbuzz.c
R "int main() { int x=0; if (1) x=5; return x; }" 5
R "int main() { int x=7; if (0) x=5; else x=9; return x; }" 9
R "int main() { int s=0; for (int i=0;i<5;i=i+1) s=s+i; return s; }" 10
# R3 cont: do/while (check-at-bottom; body runs at least once)
R "int main() { int i=0; int s=0; do { s=s+i; i=i+1; } while (i<5); return s; }" 10
R "int main() { int x=0; do { x=5; } while (0); return x; }" 5
G="../../2026-06-28-c-parity/gauntlet"
./build/cc.bin < "$G/fizzbuzz.c" | ./build/cf.bin > build/fb; chmod +x build/fb
set +e; ./build/fb; r="$?"; set -e; [ "$r" = "158" ] || { echo "gauntlet fizzbuzz.c -> $r != 158" >&2; exit 1; }
# R5 (c-parity): char escapes -- file-based (backslashes survive)
cat > build/esc.c <<'ESC'
int main() { char nl = '\n'; char z = '\0'; char t = '\t'; char a = 'A'; return a + nl + z + t; }
ESC
./build/cc.bin < build/esc.c | ./build/cf.bin > build/eo; chmod +x build/eo
set +e; ./build/eo; r="$?"; set -e; [ "$r" = "84" ] || { echo "char escapes -> $r != 75" >&2; exit 1; }
# R6 (c-parity): uninitialized decls + multiple declarators
R "int main() { int x; x = 5; return x; }" 5
R "int main() { int a, b; a = 1; b = 2; return a + b; }" 3
R "int main() { int a = 1, b = 2, c = 3; return a + b + c; }" 6
# R7 (c-parity): struct declaration + member access (.)
cat > build/st.c <<'STC'
struct Point { int x; int y; };
int main() { struct Point p; p.x = 3; p.y = 4; return p.x + p.y; }
STC
./build/cc.bin < build/st.c | ./build/cf.bin > build/sto; chmod +x build/sto
set +e; ./build/sto; r="$?"; set -e; [ "$r" = "7" ] || { echo "struct . -> $r != 7" >&2; exit 1; }
cat > build/st2.c <<'STC'
struct Rect { int w; int h; int area; };
int main() { struct Rect r; r.w = 6; r.h = 7; r.area = r.w * r.h; return r.area; }
STC
./build/cc.bin < build/st2.c | ./build/cf.bin > build/sto2; chmod +x build/sto2
set +e; ./build/sto2; r="$?"; set -e; [ "$r" = "42" ] || { echo "struct 3-field -> $r != 42" >&2; exit 1; }
cat > build/arrow.c <<'STC'
struct Point { int x; int y; };
int main() { struct Point a; a.x = 3; a.y = 4; return dist2(&a); }
int dist2(struct Point *p) { return p->x * p->x + p->y * p->y; }
STC
./build/cc.bin < build/arrow.c | ./build/cf.bin > build/aro; chmod +x build/aro
set +e; ./build/aro; r="$?"; set -e; [ "$r" = "25" ] || { echo "struct -> / *param / & -> $r != 25" >&2; exit 1; }
# R7/R6: main-anywhere reorder -> the LITERAL gauntlet point.c (helper before main)
G2="../../2026-06-28-c-parity/gauntlet"
./build/cc.bin < "$G2/point.c" | ./build/cf.bin > build/pt; chmod +x build/pt
set +e; ./build/pt; r="$?"; set -e; [ "$r" = "25" ] || { echo "gauntlet point.c -> $r != 25" >&2; exit 1; }
# R5 strings: the LITERAL gauntlet strlen.c (string literal + char* + loadb)
./build/cc.bin < "$G2/strlen.c" | ./build/cf.bin > build/sl; chmod +x build/sl
set +e; ./build/sl; r="$?"; set -e; [ "$r" = "5" ] || { echo "gauntlet strlen.c -> $r != 5" >&2; exit 1; }
# R5: string escapes inside "..." (decoded; escapes shorten the byte length)
cat > build/esc2.c <<'ESC'
int slen(char *s) { int n=0; while (*s != 0) { n++; s++; } return n; }
int main() { char *m = "a\nb"; return slen(m); }
ESC
./build/cc.bin < build/esc2.c | ./build/cf.bin > build/eo2; chmod +x build/eo2
set +e; ./build/eo2; r="$?"; set -e; [ "$r" = "3" ] || { echo "string escape -> $r != 3" >&2; exit 1; }
# R8: enum constants (auto-increment + explicit values)
cat > build/enum.c <<'EN'
enum { RED, GREEN, BLUE };
enum { T_NUM = 1, T_ADD = 2, T_MUL = 4 };
int main() { int t = T_ADD; for (int i = 0; i < T_MUL; i++) { t += T_NUM; } return t + GREEN; }
EN
./build/cc.bin < build/enum.c | ./build/cf.bin > build/eno; chmod +x build/eno
set +e; ./build/eno; r="$?"; set -e; [ "$r" = "7" ] || { echo "enum -> $r != 7" >&2; exit 1; }
# R8: typedef (scalar + named struct)
cat > build/td.c <<'TD'
struct Point { int x; int y; };
typedef struct Point Point;
typedef int Word;
int main() { Point p; p.x = 3; p.y = 4; Word w = 10; return p.x + p.y + w; }
TD
./build/cc.bin < build/td.c | ./build/cf.bin > build/tdo; chmod +x build/tdo
set +e; ./build/tdo; r="$?"; set -e; [ "$r" = "17" ] || { echo "typedef -> $r != 17" >&2; exit 1; }
# cf break: brk + endloop (first cf backend extension since rung 3)
R "int main() { int i=0; while (i<10) { if (i==3) break; i=i+1; } return i; }" 3
R "int main() { int s=0; for (int i=0;i<100;i++) { if (i==5) break; s+=i; } return s; }" 10
R "int main() { int i=0; do { if (i==4) break; i++; } while (i<100); return i; }" 4
R "int main() { int c=0; for (int i=0;i<3;i++) { for (int j=0;j<10;j++) { if (j==2) break; c++; } } return c; }" 6
R "int main() { int f=0; for (int i=1;i<=20;i++) { if (i*i==49) { f=i; break; } } return f; }" 7
# cf continue: cnt + cont marker (forward-chained, backpatched at the continue point)
R "int main() { int s=0; for(int i=0;i<5;i++){ if(i==2) continue; s+=i; } return s; }" 8
R "int main() { int s=0; int i=0; while(i<10){ i++; if(i%2==0) continue; s+=i; } return s; }" 25
R "int main() { int s=0; for(int i=0;i<100;i++){ if(i==3) continue; if(i==6) break; s+=i; } return s; }" 12
# GLOBALS (cf memsz extended) + void + bare-call statements
cat > build/gl.c <<'GL'
int total = 0;
int count;
void tick() { count = count + 1; }
int add(int n) { total = total + n; return 0; }
int main() { count = 0; add(10); add(20); tick(); tick(); return total + count; }
GL
./build/cc.bin < build/gl.c | ./build/cf.bin > build/glo; chmod +x build/glo
set +e; ./build/glo; r="$?"; set -e; [ "$r" = "32" ] || { echo "globals -> $r != 32" >&2; exit 1; }
# unbraced nested control (parse_stmt callable refactor)
R "int main() { int a[5]; a[0]=5;a[1]=2;a[2]=8;a[3]=1;a[4]=9; for(int i=0;i<5;i++) for(int j=0;j<4;j++) if(a[j]>a[j+1]){int t=a[j];a[j]=a[j+1];a[j+1]=t;} return a[0]*10+a[4]; }" 19
R "int main() { int c=0; for(int i=0;i<10;i++) if(i%2==0) c++; return c; }" 5
# struct array fields (multi-slot layout + member-array access)
cat > build/saf.c <<'SAF'
struct Stack { int data[8]; int top; };
int main() { struct Stack s; s.top=0;
  s.data[0]=10; s.top=s.top+1; s.data[1]=20; s.top=s.top+1; s.data[2]=30; s.top=s.top+1;
  int sum=0; for (int i=0;i<s.top;i++) sum=sum+s.data[i]; return sum; }
SAF
./build/cc.bin < build/saf.c | ./build/cf.bin > build/safo; chmod +x build/safo
set +e; ./build/safo; r="$?"; set -e; [ "$r" = "60" ] || { echo "struct array field -> $r != 60" >&2; exit 1; }
# char* deref-width tag: local char* digit scanning (loadb, not 4-byte load)
R "int main(){ char* p=\"42\"; int v=0; while(*p>='0' && *p<='9'){ v=v*10+(*p-'0'); p++; } return v; }" 42
R "int main(){ char* p=\"AB\"; if(*p<=200) return 1; return 0; }" 1
# global compound write-back (++ / += / -= on globals)
R "int main(){ int g; g=5; g++; g++; return g; }" 7
R "char* p; int main(){ p=\"42\"; p++; return *p; }" 50
# ★ Phase-0 capstone: the recursive-descent evaluator compiles + runs correctly
cp ../../2026-06-28-c-parity/phase0/calc.c build/calc.c
./build/cc.bin < build/calc.c | ./build/cf.bin > build/calc; chmod +x build/calc
set +e; ./build/calc; r="$?"; set -e; [ "$r" = "13" ] || { echo "calc.c (2+3*4-1) -> $r != 13" >&2; exit 1; }
rm -rf build
echo "C front-end rungs 1-6 + R1/R2 verified: pointers + functions + control flow -> native ELF, no cc"
