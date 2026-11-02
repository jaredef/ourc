struct scm { int car; struct scm *cdr; };
typedef struct scm *SCM;
struct scm cell;
struct scm tail;
int main() {
  SCM x;
  SCM y;
  SCM z;
  x = &cell;
  y = &tail;
  x->car = 10;
  x->cdr = y;
  y->car = 32;
  z = x->cdr;
  return x->car + z->car;
}
