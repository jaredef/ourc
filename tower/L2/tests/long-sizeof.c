struct s { long a; long b; char c; };
struct s g;
typedef long off_t;
int main() {
  struct s *p; p = &g;
  p->a = 100; p->b = 200; p->c = 7;
  off_t z; z = p->a + p->b;
  return sizeof(struct s) + sizeof(long) + sizeof(char) + z + p->c;
}
