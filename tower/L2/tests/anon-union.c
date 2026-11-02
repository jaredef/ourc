struct cell { long type; union { long a; char *s; }; union { long b; long c; }; };
struct cell g;
int main() { struct cell *p; p = &g; p->type = 1; p->a = 40; p->b = 2; return p->type + p->a + p->b; }
