/* entry-contract getenv: reads the marshalled envp[] at 0x10100000 (cf materializes it there),
 * bypassing Mes's unset `environ` (its crt __init_io never runs on the derived chain).
 * e[k] uses the char** subscript (8-byte load8); s[i] is a byte; the return is a low (<4GB) ptr. */
char *getenv (char *name) { char **e; char *s; int i; int k;
  e = 269484032;                                   /* 0x10100000 */
  k = 0;
  while (e[k] != 0) {
    s = e[k]; i = 0;
    while (name[i] != 0 && name[i] == s[i]) { i = i + 1; }
    if (name[i] == 0 && s[i] == 61) { return s + i + 1; }   /* "NAME=" -> value */
    k = k + 1; }
  return 0; }
