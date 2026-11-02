/* A real stdio program the derived tcc compiles host-free (rzcpp preprocess + derived tcc),
   producing an object that links (system ld, per the founding add(3,4) milestone) and runs
   BYTE-IDENTICAL to gcc. Uses #include with era-appropriate headers -- no __gnuc_va_list. */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct Point { int x; int y; } Point;

int fib(int n) { if (n < 2) return n; return fib(n - 1) + fib(n - 2); }
int dist2(Point *p) { return p->x * p->x + p->y * p->y; }

int main(int argc, char **argv) {
  int i;
  int sum = 0;
  Point p; p.x = 3; p.y = 4;

  printf("argc = %d\n", argc);
  for (i = 1; i < argc; i++) sum += atoi(argv[i]);
  printf("sum of args = %d\n", sum);
  printf("dist2(3,4)  = %d\n", dist2(&p));
  for (i = 0; i < 10; i++) printf("fib(%d) = %d (0x%x)\n", i, fib(i), fib(i));
  for (i = 0; i < 5; i++) putchar('=');
  putchar('\n');
  puts("done.");
  return sum % 7;
}
