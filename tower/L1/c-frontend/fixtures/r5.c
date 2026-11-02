int main() { return fact(5); }
int fact(int n) { if (n < 2) { return 1; } return n * fact(n - 1); }
