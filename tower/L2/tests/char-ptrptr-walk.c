/* R149: `char **e; e++` / `++e` must stride by 8 (pointee is a char*, a pointer),
   not 1. The postfix (parse_atom ~822) and pre-inc (~528) stride computations
   checked `char` before `ptr-depth>=2`, so a char** wrongly strided by 1 (argv/
   environ walking read mid-pointer -> early loop exit). Fixed: ptr-depth>=2 first. */
int main(){
  char *a[4]; a[0]="A"; a[1]="B"; a[2]="C"; a[3]=0;
  char **e=a; int n=0;
  while (*e){ n++; e++; }        /* postfix e++ on char** */
  char **f=a; int m=0;
  while (*f){ m++; ++f; }        /* pre-inc ++f on char** */
  return n*10 + m;               /* 33 */
}
