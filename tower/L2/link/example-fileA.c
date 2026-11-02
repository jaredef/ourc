extern int shared;
extern int addg(int x);
int main() { shared = 100; return addg(23) + shared; }
