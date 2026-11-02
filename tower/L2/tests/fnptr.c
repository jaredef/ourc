int add1(int x) { return x + 1; }
int dbl(int x) { return x + x; }
int apply(int f, int v) { return f(v); }
int main() {
  int g; g = add1;
  return apply(g, 10) + apply(dbl, 20);
}
