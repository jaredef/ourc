#!/bin/bash
# three-way/battery.sh -- a fixed 3-way differential battery (gcc / stock tcc-0.9.27 / derived tcc).
#   STOCK=<stock tcc> DTCC=<derived tcc.elf> bash battery.sh
set -u
HERE=$(cd "$(dirname "$0")" && pwd)
C(){ STOCK="$STOCK" DTCC="$DTCC" GCC="${GCC:-gcc}" bash "$HERE/compare.sh" "$@"; }
C "arithmetic"     'int main(){int a=17;int b=5;return a*b%13+a/b;}'
C "fib(15)"        'int fib(int n){if(n<2)return n;return fib(n-1)+fib(n-2);}int main(){return fib(15)%256;}'
C "bubble sort"    'int main(){int a[6];a[0]=9;a[1]=3;a[2]=7;a[3]=1;a[4]=8;a[5]=2;int i;int j;for(i=0;i<6;i++)for(j=0;j<5;j++)if(a[j]>a[j+1]){int t=a[j];a[j]=a[j+1];a[j+1]=t;}return a[0]*10+a[5];}'
C "struct+ptr"     'typedef struct P{int x;int y;}P;int f(P*p){return p->x*p->x+p->y*p->y;}int main(){P p;p.x=6;p.y=8;return f(&p)%256;}'
C "switch 12-case" 'int f(int x){switch(x){case 0:return 1;case 5:return 55;case 11:return 121;default:return 99;}}int main(){return f(11)%256;}'
C "union"          'typedef union U{int i;char c[4];}U;int main(){U u;u.i=0x41424344;return u.c[0];}'
C "func pointer"   'int add(int a,int b){return a+b;}int mul(int a,int b){return a*b;}int main(){int(*f[2])(int,int);f[0]=add;f[1]=mul;return f[0](3,4)+f[1](3,4);}'
C "2d array"       'int main(){int m[3][3];int i;int j;for(i=0;i<3;i++)for(j=0;j<3;j++)m[i][j]=i*3+j;return m[2][2]+m[1][1];}'
C "static local"   'int c(){static int n=0;n++;return n;}int main(){c();c();return c();}'
C "nested struct"  'typedef struct I{int v;}I;typedef struct O{I inner;int w;}O;int main(){O o;o.inner.v=5;o.w=3;return o.inner.v*o.w;}'
