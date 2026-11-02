typedef unsigned size_t;
typedef long ssize_t;
extern int notused;
struct buf { char *p; long n; void *owner; };
struct buf gb;
unsigned add(unsigned a, int b) { return a + b; }
long pick(char const *s, short k) { return s[k]; }
int main() {
  struct buf *q;
  char **pp;
  size_t z;
  q = &gb;
  q->n = 7;
  z = add(40, 2);
  return z + pick("XYZ", 1) - q->n + 5;
}
