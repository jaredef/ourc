int main() {
  int i; int s; s = 0;
  for (i = 0; i < 5; i = i + 1) { s = s + i; }
  int j; int t; t = 1;
  for (j = 1; j <= 4; j = j + 1) t = t * j;
  return s + t;
}
