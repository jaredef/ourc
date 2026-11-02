/* rzrs-seed freestanding libc shim -- ccc-subset C, __syscall-backed. Supplies exactly what rzrs.c uses:
 * printf/fprintf (%d/%u/%s), fwrite/fread/read (fd-based), strlen/strncmp/strcmp/memcpy, exit. No malloc
 * (rzrs.c uses static buffers). va_list = long[2] via ccc's __va_start/__va_next. */
typedef long va_list[2];
char _nb[24];
char _ob[1];
int strlen(char *s){ int i; i=0; while(s[i]) i=i+1; return i; }
int strncmp(char *a, char *b, int n){ int i; i=0; while(i<n){ if(a[i]!=b[i]) return a[i]-b[i]; if(a[i]==0) return 0; i=i+1; } return 0; }
int strcmp(char *a, char *b){ int i; i=0; while(a[i] && a[i]==b[i]) i=i+1; return a[i]-b[i]; }
int memcpy(char *d, char *s, int n){ int i; i=0; while(i<n){ d[i]=s[i]; i=i+1; } return 0; }
void exit(int c){ __syscall(60, c, 0, 0); }
int read(int fd, char *p, int n){ return __syscall(0, fd, p, n); }
int fwrite(char *p, int sz, int n, int fd){ __syscall(1, fd, p, sz*n); return n; }
int fread(char *p, int sz, int n, int fd){ int r; r=__syscall(0, fd, p, sz*n); return r/sz; }
void _w1(int fd, int c){ _ob[0]=c; __syscall(1, fd, _ob, 1); }
void _pdec(int fd, long v, int sign){ int i; long u;
  if(sign && v<0){ _w1(fd,45); u = 0-v; } else u = v;
  if(u==0){ _w1(fd,48); return; }
  i=0; while(u>0){ _nb[i]=48+(u-(u/10)*10); u=u/10; i=i+1; }
  while(i>0){ i=i-1; _w1(fd,_nb[i]); } }
int _vfmt(int fd, char *fmt, va_list ap){ int i; long v; char *s; i=0;
  while(fmt[i]){
    if(fmt[i]==37){ i=i+1;
      if(fmt[i]==100){ v=__va_next(ap); _pdec(fd,v,1); }
      else if(fmt[i]==117){ v=__va_next(ap); _pdec(fd,v,0); }
      else if(fmt[i]==115){ v=__va_next(ap); s=(char*)v; __syscall(1,fd,s,strlen(s)); }
      else { _w1(fd, fmt[i]); }
      i=i+1;
    } else { _w1(fd, fmt[i]); i=i+1; }
  }
  return 0; }
int printf(char *fmt, ...){ va_list ap; __va_start(ap); return _vfmt(1, fmt, ap); }
int fprintf(int fd, char *fmt, ...){ va_list ap; __va_start(ap); return _vfmt(fd, fmt, ap); }
