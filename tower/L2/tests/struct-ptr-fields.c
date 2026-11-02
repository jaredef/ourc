struct Node { int v; char c; struct Node *next; };
struct Node a;
struct Node b;
int main() {
  struct Node *p;
  struct Node *q;
  struct Node *r;
  p = &a;
  q = &b;
  p->v = 42;
  p->c = 65;
  p->next = q;
  q->v = 8;
  r = p->next;
  return p->v + p->c + r->v;
}
