extern char **environ;
extern int __stdin;
extern int __stdout;
extern int __stderr;

void __init_io ();
int eputs (char const *s);
int puts (char const *s);
int oputs (char const *s);
typedef unsigned size_t;
typedef long ssize_t;







typedef int error_t;
extern int errno;






void _exit (int status);
void exit (int status);
size_t strlen (char const *s);
ssize_t _write ();
ssize_t write (int filedes, void const *buffer, size_t size);


extern char *__brk;

char *cast_intp_to_charp (int const *i);
char *cast_long_to_charp (long i);
long cast_charp_to_long (char const *);
long cast_int_to_long (int i);
long cast_voidp_to_long (void const *);

int __mes_debug ();
void __ungetc_init ();
void __ungetc_clear (int filedes);
void __ungetc_set (int filedes, int c);
int __ungetc_p (int filedes);
long abtol (char const **p, int base);
char *itoa (int number);
char *ltoa (long number);
char *ltoab (long x, int base);
char *ntoab (long number, unsigned base, int signed_p);
char *utoa (unsigned number);
int eputc (int c);
int fdgetc (int fd);
char * fdgets (char *s, int count, int fd);
int fdputc (int c, int fd);
int fdputs (char const *s, int fd);
int fdungetc (int c, int fd);
char *_getcwd (char *buffer, size_t size);
int ioctl3 (int filedes, size_t command, long data);
int isnumber (int c, int base);
int mes_open (char const *file_name, int flags, int mask);
int _open2 (char const *file_name, int flags);
int _open3 (char const *file_name, int flags, int mask);
int oputc (int c);
int oputs (char const *s);
char *search_path (char const *file_name);
ssize_t _read (int fd, void *buffer, size_t size);
void assert_msg (int check, char *msg);

long __mesabi_imod (long a, long b);
long __mesabi_idiv (long a, long b);
void *__memcpy (void *dest, void const *src, size_t n);
void *__memmove (void *dest, void const *src, size_t n);
void *__memset (void *s, int c, size_t n);


void __assert_fail (char const *s, char const *file, unsigned line,
                    char const *function);
ssize_t __buffered_read (int filedes, void *buffer, size_t size);
size_t __buffered_read_clear (int filedes);
long brk (void *addr);

int __raise (int signal);
typedef long clock_t;





typedef long dev_t;





typedef long FILE;





typedef unsigned gid_t;






typedef unsigned ino_t;
typedef unsigned ino64_t;
typedef long intptr_t;


typedef unsigned uintptr_t;
typedef long off_t;







typedef unsigned off64_t;
typedef int pid_t;







typedef long ptrdiff_t;






typedef long sigval_t;
typedef unsigned uid_t;







typedef int wchar_t;
typedef struct scm* SCM;
typedef SCM (*FUNCTION) (void);
typedef SCM (*function0_t) (void);
typedef SCM (*function1_t) (SCM);
typedef SCM (*function2_t) (SCM, SCM);
typedef SCM (*function3_t) (SCM, SCM, SCM);
typedef SCM (*functionn_t) (SCM);

struct scm
{
  long type;
  union
  {
    struct scm *car;
    long car_value;
    char *bytes;
    long length;
    struct scm *ref;
    struct scm *binding;
    struct scm *macro;
    long port;
  };
  union
  {
    struct scm *cdr;
    long cdr_value;
    struct scm *closure;
    struct scm *continuation;
    char *name;
    struct scm *string;
    struct scm *structure;
    long value;
    FUNCTION function;
    struct scm *vector;
    long lexical_p;
  };
};
extern char *g_datadir;
extern int g_debug;
extern char *g_buf;
extern int g_continuations;
extern struct scm *g_symbols;
extern struct scm *g_symbol_max;
extern int g_mini;


extern struct scm *R0;

extern struct scm *R1;

extern struct scm *R2;

extern struct scm *R3;

extern struct scm *M0;

extern struct scm *M1;

extern struct scm *g_macros;
extern struct scm *g_ports;


extern size_t ARENA_SIZE;
extern size_t MAX_ARENA_SIZE;
extern size_t STACK_SIZE;
extern size_t JAM_SIZE;
extern size_t GC_SAFETY;
extern size_t MAX_STRING;
extern char *g_arena;
extern struct scm *cell_arena;
extern struct scm *cell_zero;

extern struct scm *g_free;
extern struct scm *g_symbol;

extern struct scm **g_stack_array;
extern struct scm *g_cells;
extern struct scm *g_news;
extern long g_stack;
extern size_t gc_count;
extern struct timespec *gc_start_time;
extern struct timespec *gc_end_time;
extern size_t gc_time;

extern char **__execl_c_argv;
extern char **__execle_c_env;
extern char *__open_boot_buf;
extern char *__open_boot_file_name;
extern char *__setenv_buf;
extern char *__reader_read_char_buf;
extern struct timespec *g_start_time;
extern struct timeval *__gettimeofday_time;
extern struct timespec *__get_internal_run_time_ts;
extern struct utsname *__uts;

extern struct scm *scm_hash_table_type;
extern struct scm *scm_variable_type;

struct scm *cast_charp_to_scmp (char const *i);
struct scm **cast_charp_to_scmpp (char const *i);
char *cast_voidp_to_charp (void const *i);
long cast_scmp_to_long (struct scm *i);
char *cast_scmp_to_charp (struct scm *i);

struct scm *alloc (long n);
struct scm *apply (struct scm *f, struct scm *x, struct scm *a);
struct scm *apply_builtin (struct scm *fn, struct scm *x);
struct scm *apply_builtin0 (struct scm *fn);
struct scm *apply_builtin1 (struct scm *fn, struct scm *x);
struct scm *apply_builtin2 (struct scm *fn, struct scm *x, struct scm *y);
struct scm *apply_builtin3 (struct scm *fn, struct scm *x, struct scm *y, struct scm *z);
struct scm *builtin_name (struct scm *builtin);
struct scm *cstring_to_list (char const *s);
struct scm *cstring_to_symbol (char const *s);
struct scm *cell_ref (struct scm *cell, long index);
struct scm *current_module_variable (struct scm *name, struct scm *define_p);
struct scm *standard_eval_closure (struct scm *name, struct scm *define_p);
struct scm *standard_interface_eval_closure (struct scm *name, struct scm *define_p);
struct scm *module_make_local_var_x (struct scm *module, struct scm *name);
struct scm *module_variable (struct scm *module, struct scm *name);
struct scm *fdisplay_ (struct scm *, int, int);
struct scm *init_symbols ();
struct scm *init_time (struct scm *a);
struct scm *lookup_binding (struct scm *name, struct scm *define_p);
struct scm *lookup_value (struct scm *name);
struct scm *make_builtin_type ();
struct scm *make_bytes (size_t length);
struct scm *make_cell (long type, struct scm *car, struct scm *cdr);
struct scm *make_pointer_cell (long type, long car, void *cdr);
struct scm *make_value_cell (long type, long car, long cdr);
struct scm *make_char (int n);
struct scm *make_continuation (long n);
struct scm *make_hash_table_ (long size);
struct scm *make_hash_table_type ();
struct scm *make_initial_module (struct scm *a);
struct scm *make_macro (struct scm *name, struct scm *x);
struct scm *make_number (long n);
struct scm *make_ref (struct scm *x);
struct scm *make_string (char const *s, size_t length);
struct scm *make_string0 (char const *s);
struct scm *make_string_port (struct scm *x);
struct scm *make_variable_type ();
struct scm *make_vector_ (long k, struct scm *e);
struct scm *mes_builtins (struct scm *a);
struct scm *push_cc (struct scm *p1, struct scm *p2, struct scm *a, struct scm *c);
struct scm *set_x (struct scm *x, struct scm *e, int define_p);
struct scm *struct_ref_ (struct scm *x, long i);
struct scm *struct_set_x_ (struct scm *x, long i, struct scm *e);
struct scm *vector_ref_ (struct scm *x, long i);
struct scm *vector_set_x_ (struct scm *x, long i, struct scm *e);
FUNCTION builtin_function (struct scm *builtin);
void assert_num (long pos, struct scm *x);
void assert_range (int assert, long x);
void assert_struct (long pos, struct scm *x);



char *cell_bytes (struct scm *x);
int peekchar ();
int readchar ();
int unreadchar ();
long gc_free ();
long length__ (struct scm *x);
size_t bytes_cells (size_t length);
void assert_max_string (size_t i, char const *msg, char const *string);
void assert_msg (int check, char *msg);
void assert_number (char const *name, struct scm *x);
void copy_cell (struct scm *to, struct scm *from);
void gc_ ();
void gc_dump_arena (struct scm *cells, long size);
void gc_init ();
void gc_peek_frame ();
void gc_pop_frame ();
void gc_push_frame ();
void gc_stats_ (char const* where);
void init_symbols_ ();
long seconds_and_nanoseconds_to_long (long s, long ns);

struct scm *make_builtin (struct scm *builtin_type, struct scm *name, struct scm *arity, struct scm *function);
struct scm *builtin_name (struct scm *builtin);
struct scm *builtin_arity (struct scm *builtin);
struct scm *builtin_p (struct scm *x);
struct scm *builtin_printer (struct scm *builtin);

struct scm *car (struct scm *x);
struct scm *cdr (struct scm *x);
struct scm *list (struct scm *x);
struct scm *null_p (struct scm *x);
struct scm *eq_p (struct scm *x, struct scm *y);
struct scm *values (struct scm *x);
struct scm *acons (struct scm *key, struct scm *value, struct scm *alist);
struct scm *length (struct scm *x);
struct scm *error (struct scm *key, struct scm *x);
struct scm *append2 (struct scm *x, struct scm *y);
struct scm *append_reverse (struct scm *x, struct scm *y);
struct scm *reverse_x_ (struct scm *x, struct scm *t);
struct scm *assq (struct scm *x, struct scm *a);
struct scm *assoc (struct scm *x, struct scm *a);

struct scm *display_ (struct scm *x);
struct scm *display_error_ (struct scm *x);
struct scm *display_port_ (struct scm *x, struct scm *p);
struct scm *write_ (struct scm *x);
struct scm *write_error_ (struct scm *x);
struct scm *write_port_ (struct scm *x, struct scm *p);

struct scm *pairlis (struct scm *x, struct scm *y, struct scm *a);
struct scm *set_car_x (struct scm *x, struct scm *e);
struct scm *set_cdr_x (struct scm *x, struct scm *e);
struct scm *add_formals (struct scm *formals, struct scm *x);
struct scm *eval_apply ();
struct scm *primitive_load (struct scm *filename);
struct scm *make_binding (struct scm *name, struct scm *variable);

struct scm *gc_stats ();
struct scm *cons (struct scm *x, struct scm *y);
struct scm *gc_check ();
struct scm *gc ();

struct scm *hashq (struct scm *x, struct scm *size);
struct scm *hash (struct scm *x, struct scm *size);
struct scm *hashq_get_handle (struct scm *table, struct scm *key);
struct scm *hashq_ref_ (struct scm *table, struct scm *key, struct scm *dflt);
struct scm *hash_ref_ (struct scm *table, struct scm *key, struct scm *dflt);
struct scm *hashq_create_handle_x (struct scm *table, struct scm *key, struct scm *init);
struct scm *hash_create_handle_x (struct scm *table, struct scm *key, struct scm *init);
struct scm *hashq_set_x (struct scm *table, struct scm *key, struct scm *value);
struct scm *hash_set_x (struct scm *table, struct scm *key, struct scm *value);
struct scm *hash_remove_x (struct scm *table, struct scm *key);
struct scm *hash_table_printer (struct scm *table);
struct scm *make_hash_table (struct scm *x);
struct scm *hash_buckets (struct scm *table);
struct scm *hash_table_p (struct scm *x);
struct scm *hash_clear_x (struct scm *table);

struct scm *type_ (struct scm *x);
struct scm *car_ (struct scm *x);
struct scm *cdr_ (struct scm *x);
struct scm *xassq (struct scm *x, struct scm *a);
struct scm *memq (struct scm *x, struct scm *a);
struct scm *equal2_p (struct scm *a, struct scm *b);
struct scm *last_pair (struct scm *x);
struct scm *pair_p (struct scm *x);
struct scm *char_to_integer (struct scm *x);
struct scm *integer_to_char (struct scm *x);
struct scm *make_bytevector (struct scm *args);
struct scm *bytevector_u8_ref (struct scm *bv, struct scm *k);
struct scm *bytevector_u8_set_x (struct scm *bv, struct scm *k, struct scm *value);

struct scm *greater_p (struct scm *x);
struct scm *less_p (struct scm *x);
struct scm *is_p (struct scm *x);
struct scm *minus (struct scm *x);
struct scm *plus (struct scm *x);
struct scm *divide (struct scm *x);
struct scm *modulo (struct scm *a, struct scm *b);
struct scm *multiply (struct scm *x);
struct scm *logand (struct scm *x);
struct scm *logior (struct scm *x);
struct scm *lognot (struct scm *x);
struct scm *logxor (struct scm *x);
struct scm *ash (struct scm *n, struct scm *count);

struct scm *initial_module ();
struct scm *current_module ();
struct scm *set_current_module (struct scm *module);

struct scm *abort_ ();
struct scm *exit_ (struct scm *x);
struct scm *peek_byte ();
struct scm *read_byte ();
struct scm *unread_byte (struct scm *i);
struct scm *peek_char ();
struct scm *read_char (struct scm *port);
struct scm *unread_char (struct scm *i);
struct scm *write_char (struct scm *i);
struct scm *write_byte (struct scm *x);
struct scm *getenv_ (struct scm *s);
struct scm *setenv_ (struct scm *s, struct scm *v);
struct scm *access_p (struct scm *file_name, struct scm *mode);
struct scm *current_input_port ();
struct scm *open_ (struct scm *file_name, struct scm *flags);
struct scm *open_input_file (struct scm *file_name);
struct scm *open_input_string (struct scm *string);
struct scm *set_current_input_port (struct scm *port);
struct scm *current_output_port ();
struct scm *current_error_port ();
struct scm *open_output_file (struct scm *x);
struct scm *set_current_output_port (struct scm *port);
struct scm *set_current_error_port (struct scm *port);
struct scm *chmod_ (struct scm *file_name, struct scm *mode);
struct scm *isatty_p (struct scm *port);
struct scm *primitive_fork ();
struct scm *primitive_exit (struct scm *status);
struct scm *execl_ (struct scm *file_name, struct scm *args);
struct scm *execle_ (struct scm *file_name, struct scm *args, struct scm *env);
struct scm *waitpid_ (struct scm *pid, struct scm *options);
struct scm *current_time ();
struct scm *gettimeofday_ ();
struct scm *get_internal_run_time ();
struct scm *getcwd_ ();
struct scm *dup_ (struct scm *port);
struct scm *dup2_ (struct scm *old, struct scm *new);
struct scm *delete_file (struct scm *file_name);
struct scm *uname_ ();

struct scm *read_input_file_env_ (struct scm *e, struct scm *a);
struct scm *read_input_file_env (struct scm *a);
struct scm *read_env (struct scm *a);
struct scm *reader_read_sexp (struct scm *c, struct scm *s, struct scm *a);
struct scm *reader_read_character ();
struct scm *reader_read_binary ();
struct scm *reader_read_octal ();
struct scm *reader_read_hex ();
struct scm *reader_read_string ();

struct scm *frame_printer (struct scm *frame);
struct scm *make_stack (struct scm *stack);
struct scm *stack_length (struct scm *stack);
struct scm *stack_ref (struct scm *stack, struct scm *index);

struct scm *string_equal_p (struct scm *a, struct scm *b);
struct scm *symbol_to_string (struct scm *symbol);
struct scm *symbol_to_keyword (struct scm *symbol);
struct scm *keyword_to_string (struct scm *keyword);
struct scm *string_to_symbol (struct scm *string);
struct scm *make_symbol (struct scm *string);
struct scm *string_to_list (struct scm *string);
struct scm *list_to_string (struct scm *list);
struct scm *read_string (struct scm *port);
struct scm *string_append (struct scm *x);
struct scm *string_length (struct scm *string);
struct scm *string_ref (struct scm *str, struct scm *k);
struct scm *string_set_x (struct scm *str, struct scm *k, struct scm *c);

struct scm *make_struct (struct scm *type, struct scm *fields, struct scm *printer);
struct scm *struct_length (struct scm *x);
struct scm *struct_ref (struct scm *x, struct scm *i);
struct scm *struct_set_x (struct scm *x, struct scm *i, struct scm *e);

struct scm *make_variable (struct scm *var);
struct scm *variable_p (struct scm *x);
struct scm *variable_ref (struct scm *var);
struct scm *variable_set_x (struct scm *var, struct scm *value);
struct scm *variable_printer (struct scm *var);

struct scm *make_vector (struct scm *x);
struct scm *vector_length (struct scm *x);
struct scm *vector_ref (struct scm *x, struct scm *i);
struct scm *vector_entry (struct scm *x);
struct scm *vector_set_x (struct scm *x, struct scm *i, struct scm *e);
struct scm *list_to_vector (struct scm *x);
struct scm *vector_to_list (struct scm *v);
struct scm *getpid_ ();
struct scm *environ_ (struct scm *args);
struct scm *opendir_ (struct scm *args);
struct scm *closedir_ (struct scm *args);
struct scm *readdir_ (struct scm *args);
struct scm *pipe_ ();
struct scm *close_port (struct scm *port);
struct scm *seek (struct scm *port, struct scm *offset, struct scm *whence);
struct scm *chdir_ (struct scm *file_name);
struct scm *stat_ (struct scm *args);
struct scm *lstat_ (struct scm *args);
struct scm *rename_file (struct scm *old_name, struct scm *new_name);
struct scm *mkdir_ (struct scm *file_name);
struct scm *rmdir_ (struct scm *file_name);
struct scm *link_ (struct scm *old_name, struct scm *new_name);
struct scm *symlink_ (struct scm *old_name, struct scm *new_name);
struct scm *umask_ (struct scm *mode);
struct scm *utime_ (struct scm *file_name, struct scm *actime, struct scm *modtime);
struct scm *sleep_ (struct scm *seconds);
extern struct scm *cell_nil;
extern struct scm *cell_f;
extern struct scm *cell_t;
extern struct scm *cell_dot;
extern struct scm *cell_arrow;
extern struct scm *cell_undefined;
extern struct scm *cell_unspecified;
extern struct scm *cell_closure;
extern struct scm *cell_circular;

extern struct scm *cell_vm_apply;
extern struct scm *cell_vm_apply2;
extern struct scm *cell_vm_begin;
extern struct scm *cell_vm_begin_eval;
extern struct scm *cell_vm_begin_expand;
extern struct scm *cell_vm_begin_expand_eval;
extern struct scm *cell_vm_begin_expand_macro;
extern struct scm *cell_vm_begin_read_input_file;
extern struct scm *cell_vm_call_with_current_continuation2;
extern struct scm *cell_vm_call_with_values2;
extern struct scm *cell_vm_eval;
extern struct scm *cell_vm_eval2;
extern struct scm *cell_vm_eval_check_func;
extern struct scm *cell_vm_eval_define;
extern struct scm *cell_vm_eval_macro_expand_eval;
extern struct scm *cell_vm_eval_macro_expand_expand;
extern struct scm *cell_vm_eval_set_x;
extern struct scm *cell_vm_evlis;
extern struct scm *cell_vm_evlis2;
extern struct scm *cell_vm_evlis3;
extern struct scm *cell_vm_if;
extern struct scm *cell_vm_if_expr;
extern struct scm *cell_vm_macro_expand;
extern struct scm *cell_vm_macro_expand_car;
extern struct scm *cell_vm_macro_expand_cdr;
extern struct scm *cell_vm_macro_expand_define;
extern struct scm *cell_vm_macro_expand_define_macro;
extern struct scm *cell_vm_macro_expand_lambda;
extern struct scm *cell_vm_macro_expand_set_x;
extern struct scm *cell_vm_return;

extern struct scm *cell_symbol_lambda;
extern struct scm *cell_symbol_begin;
extern struct scm *cell_symbol_if;
extern struct scm *cell_symbol_quote;
extern struct scm *cell_symbol_define;
extern struct scm *cell_symbol_define_macro;
extern struct scm *cell_symbol_quasiquote;
extern struct scm *cell_symbol_unquote;
extern struct scm *cell_symbol_unquote_splicing;
extern struct scm *cell_symbol_syntax;
extern struct scm *cell_symbol_quasisyntax;
extern struct scm *cell_symbol_unsyntax;
extern struct scm *cell_symbol_unsyntax_splicing;
extern struct scm *cell_symbol_set_x;
extern struct scm *cell_symbol_sc_expand;
extern struct scm *cell_symbol_macro_expand;
extern struct scm *cell_symbol_portable_macro_expand;
extern struct scm *cell_symbol_sc_expander_alist;
extern struct scm *cell_symbol_call_with_values;
extern struct scm *cell_symbol_call_with_current_continuation;
extern struct scm *cell_symbol_current_environment;
extern struct scm *cell_symbol_car;
extern struct scm *cell_symbol_cdr;
extern struct scm *cell_symbol_not_a_number;
extern struct scm *cell_symbol_not_a_pair;
extern struct scm *cell_symbol_system_error;
extern struct scm *cell_symbol_throw;
extern struct scm *cell_symbol_unbound_variable;
extern struct scm *cell_symbol_wrong_number_of_args;
extern struct scm *cell_symbol_wrong_type_arg;
extern struct scm *cell_symbol_buckets;
extern struct scm *cell_symbol_builtin;
extern struct scm *cell_symbol_frame;
extern struct scm *cell_symbol_hashq_table;
extern struct scm *cell_symbol_procedure;
extern struct scm *cell_symbol_record_type;
extern struct scm *cell_symbol_size;
extern struct scm *cell_symbol_stack;
extern struct scm *cell_symbol_argv;
extern struct scm *cell_symbol_mes_datadir;
extern struct scm *cell_symbol_mes_version;
extern struct scm *cell_symbol_internal_time_units_per_second;
extern struct scm *cell_symbol_compiler;
extern struct scm *cell_symbol_arch;
extern struct scm *cell_symbol_variable;
extern struct scm *cell_symbol_standard_eval_closure;
extern struct scm *cell_symbol_standard_interface_eval_closure;
extern struct scm *cell_type_bytes;
extern struct scm *cell_type_char;
extern struct scm *cell_type_closure;
extern struct scm *cell_type_continuation;
extern struct scm *cell_type_function;
extern struct scm *cell_type_keyword;
extern struct scm *cell_type_macro;
extern struct scm *cell_type_number;
extern struct scm *cell_type_pair;
extern struct scm *cell_type_port;
extern struct scm *cell_type_ref;
extern struct scm *cell_type_special;
extern struct scm *cell_type_string;
extern struct scm *cell_type_struct;
extern struct scm *cell_type_symbol;
extern struct scm *cell_type_values;
extern struct scm *cell_type_binding;
extern struct scm *cell_type_vector;
extern struct scm *cell_type_broken_heart;
extern struct scm *cell_symbol_program;
extern struct scm *cell_symbol_test;

int dup (int old);
int dup2 (int old, int new);
typedef unsigned char uint8_t;
typedef signed char int8_t;
typedef unsigned short uint16_t;
typedef short int16_t;
typedef unsigned uint32_t;
typedef int int32_t;

typedef unsigned long long uint64_t;
typedef long long int64_t;






typedef long intmax_t;
typedef unsigned long uintmax_t;
typedef long stack_t;
FILE *fdopen (int fd, char const *mode);
FILE *fopen (char const *file_name, char const *mode);
int eputc (int c);
int eputs (char const *s);
int fclose (FILE * stream);
int feof (FILE * stream);
int ferror (FILE * stream);
int fflush (FILE * stream);
int fgetc (FILE * stream);
char *fgets (char *s, int size, FILE * stream);
int fpurge (FILE * stream);
int fputc (int c, FILE * stream);
int fputs (char const *s, FILE * stream);
int fseek (FILE * stream, long offset, int whence);
int getc (FILE * stream);
int getchar (void);
char *getlogin (void);
int putc (int c, FILE * stream);
int putchar (int c);
int puts (char const *s);
int remove (char const *file_name);
int rename (char const *old_name, char const *new_name);
int setvbuf (FILE * stream, char *buf, int mode, size_t size);
int ungetc (int c, FILE * stream);
long ftell (FILE * stream);
size_t fread (void *ptr, size_t size, size_t count, FILE * stream);
size_t freadahead (FILE * fp);
size_t fwrite (void const *ptr, size_t size, size_t count, FILE * stream);
void perror (char const *message);
typedef int (*comparison_fn_t) (void const *, void const *);
void *alloca (size_t size);



void abort (void);
double atof (char const *s);
int atoi (char const *s);
int atexit (void (*function) (void));
void *calloc (size_t nmemb, size_t size);
void _exit (int status);
void exit (int status);
void free (void *ptr);
char *getenv (char const *s);
int putenv (char *s);
int setenv (char const *s, char const *v, int overwrite_p);
void unsetenv (char const *name);
void *malloc (size_t);
void qsort (void *base, size_t nmemb, size_t size, int (*compar) (void const *, void const *));
int rand (void);
void *realloc (void *p, size_t size);
double strtod (char const *string, char **tailptr);
float strtof (char const *string, char **tailptr);
long double strtold (char const *string, char **tailptr);
long strtol (char const *string, char **tailptr, int base);
long long strtoll (char const *string, char **tailptr, int base);
unsigned long strtoul (char const *string, char **tailptr, int base);
unsigned long long strtoull (char const *string, char **tailptr, int base);




void *bsearch (void const *key, void const *array, size_t count, size_t size, comparison_fn_t compare);
void *memchr (void const *block, int c, size_t size);
void *memcpy (void *dest, void const *src, size_t n);
void *memmove (void *dest, void const *src, size_t n);
void *memset (void *s, int c, size_t n);
int memcmp (void const *s1, void const *s2, size_t n);
void *memmem (void const *haystack, int haystack_len, void const *needle, int needle_len);
char *strcat (char *dest, char const *src);
char *strchr (char const *s, int c);
int strcasecmp (char const *s1, char const *s2);
int strcmp (char const *, char const *);
char *strcpy (char *dest, char const *src);
size_t strcspn (char const *string, char const *stopset);
size_t strlen (char const *);
char *strncat (char *to, char const *from, size_t size);
char *strncpy (char *to, char const *from, size_t size);
int strncmp (char const *, char const *, size_t);
char *strpbrk (char const *string, char const *stopset);
char *strrchr (char const *s, int c);
size_t strspn (char const *string, char const *skipset);
char *strstr (char const *haystack, char const *needle);
char *strlwr (char *string);
char *strupr (char *string);


char *strerror (int errnum);
typedef long clockid_t;
typedef long time_t;


struct tm
{
  int tm_sec;
  int tm_min;
  int tm_hour;
  int tm_mday;
  int tm_mon;
  int tm_year;
  int tm_wday;
  int tm_yday;
  int tm_isdst;
};




struct timespec
{
  long tv_sec;
  long tv_nsec;
};




int clock_gettime (clockid_t clk_id, struct timespec *tp);
struct tm *localtime (time_t const *timep);
struct tm *gmtime (time_t const *time);
time_t mktime (struct tm *broken_time);
int nanosleep (struct timespec const *requested_time, struct timespec const *remaining);
time_t time (time_t * tloc);




typedef int mode_t;


struct stat
{
  unsigned long st_dev;
  unsigned long st_ino;
  unsigned long st_nlink;
  unsigned int st_mode;
  unsigned int st_uid;
  unsigned int st_gid;
  unsigned int __pad0;
  unsigned long st_rdev;
  long st_size;
  long st_blksize;
  long st_blocks;
  unsigned long st_atime;
  unsigned long st_atime_nsec;
  unsigned long st_mtime;
  unsigned long st_mtime_nsec;
  unsigned long st_ctime;
  unsigned long st_ctime_nsec;
  unsigned long __pad1;
  unsigned long __pad2;
  unsigned long __pad3;
  unsigned long __pad4;
};

int chmod (char const *file_name, mode_t mode);
int fstat (int filedes, struct stat *buf);
int mkdir (char const *file_name, mode_t mode);
int mknod (char const *file_name, mode_t mode, dev_t dev);
int chown (char const *file_name, uid_t owner, gid_t group);
int rmdir (char const *file_name);
int stat (char const *file_name, struct stat *buf);
int lstat (char const *file_name, struct stat *buf);
mode_t umask (mode_t mask);
int utimensat(int dirfd, char const *file_name,
              struct timespec const *times, int flags);

struct timeval
{
  long tv_sec;
  long tv_usec;
};

struct timezone
{
  int tz_minuteswest;
  int tz_dsttime;
};

struct itimerval
{
  struct timeval it_interval;
  struct timeval it_value;
};





int gettimeofday (struct timeval *tv, struct timezone *tz);
int setitimer (int which, struct itimerval const *new, struct itimerval *old);

struct utsname
{
  char sysname[65];
  char nodename[65];
  char release[65];
  char version[65];
  char machine[65];



  char __domainname[65];

};

int uname (struct utsname *uts);
struct rusage
{
  struct timeval ru_utime;
  struct timeval ru_stime;
  long ru_maxrss;
  long ru_ixrss;
  long ru_idrss;
  long ru_isrss;
  long ru_minflt;
  long ru_majflt;
  long ru_nswap;
  long ru_inblock;
  long ru_oublock;
  long ru_msgsnd;
  long ru_msgrcv;
  long ru_nsignals;
  long ru_nvcsw;
  long ru_nivcsw;
};






int getrusage (int processes, struct rusage *rusage);
pid_t waitpid (pid_t pid, int *status_ptr, int options);
pid_t wait (int *status_ptr);
pid_t wait4 (pid_t pid, int *wstatus, int options,
             struct rusage *rusage);
int access (char const *s, int mode);
unsigned int alarm (unsigned int seconds);
int chdir (char const *file_name);
int close (int fd);
int execv (char const *file_name, char *const argv[]);
int execl (char const *file_name, char const *arg, ...);
int execlp (char const *file_name, char const *arg, ...);
int execve (char const *file, char *const argv[], char *const env[]);
int execvp (char const *file, char *const argv[]);
int fork (void);
int fsync (int filedes);
char *getcwd (char *buf, size_t size);
uid_t getuid (void);
gid_t getgid (void);
int setgid (gid_t newgid);
int setuid (uid_t newuid);
uid_t geteuid (void);
gid_t getegid (void);
pid_t getpgrp (void);
pid_t getpid (void);
pid_t getppid (void);
int getpgid (pid_t pid);
int isatty (int fd);
int link (char const *old_name, char const *new_name);
off_t lseek (int fd, off_t offset, int whence);
long pathconf (char const *file_name, int parameter);
int pipe (int filedes[2]);
ssize_t read (int fd, void *buffer, size_t size);
ssize_t readlink (char const *file_name, char *buffer, size_t size);
char *realpath (char const *name, char *resolved);




void *sbrk (intptr_t delta);

int symlink (char const *old_name, char const *new_name);
int unlink (char const *file_name);
ssize_t write (int filedes, void const *buffer, size_t size);

struct scm *
abort_ ()
{
  if (g_debug > 0)
    eputs ("abort!\n");
  if (__raise (6) < 0)
    {

      char* x = 0;
      x[0] = 2;
    }
  return cell_unspecified;
}

struct scm *
exit_ (struct scm *x)
{
  int status;

  if (x == cell_t)
    status = 0;
  else if (x == cell_f)
    status = 1;
  else
    {
      assert_msg (x->type == 6, "x->type == TNUMBER");
      status = x->value;
    }

  exit (status);
}

int
peekchar ()
{
  if (__stdin >= 0)
    {
      int c = readchar ();
      unreadchar (c);
      return c;
    }
  struct scm *port = current_input_port ();
  struct scm *string = port->string;
  size_t length = string->length;
  if (length == 0)
    return -1;
  char const *p = cell_bytes (string->string);
  return p[0];
}

int
readchar ()
{
  if (__stdin >= 0)
    return fdgetc (__stdin);
  struct scm *port = current_input_port ();
  struct scm *string = port->string;
  size_t length = string->length;
  if (length == 0)
    return -1;
  char const *p = cell_bytes (string->string);
  int c = p[0];
  p = p + 1;
  port->string = make_string (p, length - 1);
  return c;
}

int
unreadchar (int c)
{
  if (__stdin >= 0)
    return fdungetc (c, __stdin);
  if (c == -1)
    return c;
  struct scm *port = current_input_port ();
  struct scm *string = port->string;
  size_t length = string->length;
  char *p = cell_bytes (string->string);
  p = p - 1;
  string = make_string (p, length + 1);
  p = cell_bytes (string->string);
  p[0] = c;
  port->string = string;
  return c;
}

struct scm *
peek_byte ()
{
  return make_number (peekchar ());
}

struct scm *
read_byte ()
{
  return make_number (readchar ());
}

struct scm *
unread_byte (struct scm *i)
{
  unreadchar (i->value);
  return i;
}

struct scm *
peek_char ()
{
  return make_char (peekchar ());
}

struct scm *
read_char (struct scm *port)
{
  int fd = __stdin;
  if (port->type == 7)
    if (port->car->type == 6)
      __stdin = port->car->value;
  struct scm *c = make_char (readchar ());
  __stdin = fd;
  return c;
}

struct scm *
unread_char (struct scm *i)
{
  unreadchar (i->value);
  return i;
}

struct scm *
write_char (struct scm *i)
{
  write_byte (i);
  return i;
}

struct scm *
write_byte (struct scm *x)
{
  struct scm *c = car (x);
  struct scm *p = cdr (x);
  int fd = __stdout;
  if (p->type == 7)
    {
      struct scm *f = p->car;
      if (f->type == 6)
        {
          long v = f->value;
          if (v != 1)
            fd = v;
          if (v == 2)
            fd = __stderr;
        }
    }
  char cc = c->value;
  write (fd, &cc, 1);

  assert_msg (c->type == 6 || c->type == 0, "c->type == TNUMBER || c->type == TCHAR");

  return c;
}

struct scm *
getenv_ (struct scm *s)
{
  char *p;
  p = getenv (cell_bytes (s->string));
  if (p != 0)
    return make_string0 (p);
  return cell_f;
}

struct scm *
setenv_ (struct scm *s, struct scm *v)
{
  char *buf = __setenv_buf;
  strcpy (buf, cell_bytes (s->string));
  setenv (buf, cell_bytes (v->string), 1);
  return cell_unspecified;
}

struct scm *
access_p (struct scm *file_name, struct scm *mode)
{
  int result = access (cell_bytes (file_name->string), mode->value);
  if (result == 0)
    return cell_t;
  return cell_f;
}

struct scm *
current_input_port ()
{
  if (__stdin >= 0)
    return make_number (__stdin);
  struct scm *x = g_ports;
  struct scm *a;
  while (x != 0)
    {
      a = x->car;
      if (a->port == __stdin)
        return a;
      x = x->cdr;
    }
  return x->car;
}

struct scm *
open_ (struct scm *file_name, struct scm *flags)
{
  int mode = 00400 | 00200;
  int filedes = mes_open (cell_bytes (file_name->string),
                          flags->value,
                          mode);
  if (filedes < 0)
    error (cell_symbol_system_error,
           cons (make_string0 ("Cannot open file"), file_name));
  return make_number (filedes);
}

struct scm *
open_input_file (struct scm *file_name)
{
  int filedes = mes_open (cell_bytes (file_name->string), 0, 0);
  if (filedes == -1)
    error (cell_symbol_system_error, cons (make_string0 ("No such file or directory"), file_name));
  return make_number (filedes);
}

struct scm *
open_input_string (struct scm *string)
{
  struct scm *port = make_string_port (string);
  g_ports = cons (port, g_ports);
  return port;
}

struct scm *
set_current_input_port (struct scm *port)
{
  struct scm *prev = current_input_port ();
  if (port->type == 6)
    {
      int p = port->value;
      if (p != 0)
        __stdin = p;
      else
        __stdin = 0;
    }
  else if (port->type == 8)
    __stdin = port->port;
  return prev;
}

struct scm *
current_output_port ()
{
  return make_number (__stdout);
}

struct scm *
current_error_port ()
{
  return make_number (__stderr);
}

struct scm *
open_output_file (struct scm *x)
{
  struct scm *file_name = car (x);
  x = cdr (x);
  int mode = 00400 | 00200;
  if (x->type == 7)
    {
      struct scm *i = car (x);
      if (i->type == 6)
        mode = i->value;
    }
  return make_number (mes_open (cell_bytes (file_name->string), 1 | 0x40 | 0x200, mode));
}

struct scm *
set_current_output_port (struct scm *port)
{
  if (port->value != 0)
    __stdout = port->value;
  else
    __stdout = 1;
  return current_output_port ();
}

struct scm *
set_current_error_port (struct scm *port)
{
  if (port->value != 0)
    __stderr = port->value;
  else
    __stderr = 2;
  return current_error_port ();
}

struct scm *
chmod_ (struct scm *file_name, struct scm *mode)
{
  chmod (cell_bytes (file_name->string), mode->value);
  return cell_unspecified;
}

struct scm *
isatty_p (struct scm *port)
{
  if (isatty (port->value) != 0)
    return cell_t;
  return cell_f;
}

struct scm *
primitive_fork ()
{
  return make_number (fork ());
}

struct scm *
primitive_exit (struct scm *status)
{
  _exit (status->value);
}

struct scm *
execl_ (struct scm *file_name, struct scm *args)
{
  char *c_file_name = cell_bytes (file_name->string);

  char **c_argv = __execl_c_argv;
  int i = 0;

  if (length__ (args) > 1000)
    error (cell_symbol_system_error,
           cons (file_name, cons (make_string0 ("too many arguments"), cons (file_name, args))));

  struct scm *arg;
  while (args != cell_nil)
    {
      assert_msg (args->car->type == 11, "args->car->type == TSTRING");
      arg = args->car;
      c_argv[i] = cell_bytes (arg->string);
      i = i + 1;
      args = args->cdr;
      if (g_debug > 2)
        {
          eputs ("arg[");
          eputs (itoa (i));
          eputs ("]: ");
          eputs (c_argv[i - 1]);
          eputs ("\n");
        }
    }
  c_argv[i] = 0;

  return make_number (execv (c_file_name, c_argv));
}

struct scm *
execle_ (struct scm *file_name, struct scm *args, struct scm *env)
{
  char *c_file_name = cell_bytes (file_name->string);

  char **c_argv = __execl_c_argv;
  int i = 0;

  if (length__ (args) > 1000)
    error (cell_symbol_system_error,
           cons (make_string0 ("too many arguments"),
                 cons (file_name, args)));

  struct scm *arg;
  while (args != cell_nil)
    {
      assert_msg (args->car->type == 11, "args->car->type == TSTRING");
      arg = args->car;
      c_argv[i] = cell_bytes (arg->string);
      i = i + 1;
      args = args->cdr;
    }
  c_argv[i] = 0;

  char **c_env = __execle_c_env;
  i = 0;

  if (length__ (env) > 1000)
    error (cell_symbol_system_error,
           cons (make_string0 ("too many variables"),
                 cons (file_name, env)));

  while (env != cell_nil)
    {
      assert_msg (env->car->type == 11, "env->car->type == TSTRING");
      c_env[i] = cell_bytes (env->car->string);
      i = i + 1;
      env = env->cdr;
    }
  c_env[i] = 0;

  return make_number (execve (c_file_name, c_argv, c_env));
}

struct scm *
waitpid_ (struct scm *pid, struct scm *options)
{
  int status;
  int child = waitpid (pid->value, &status, options->value);
  return cons (make_number (child), make_number (status));
}
struct scm *
init_time (struct scm *a)
{
  clock_gettime (2, g_start_time);
  a = acons (cell_symbol_internal_time_units_per_second, make_number (1000000000), a);
}

struct scm *
current_time ()
{
  return make_number (time (0));
}

struct scm *
gettimeofday_ ()
{
  struct timeval *time = __gettimeofday_time;
  gettimeofday (time, 0);
  return cons (make_number (time->tv_sec), make_number (time->tv_usec));
}







long
seconds_and_nanoseconds_to_long (long s, long ns)
{
  size_t uns = ns;
  if (ns < 0)
    {
      uns = - ns;
      return s * 1000000000 - uns / (1000000000 / 1000000000);
    }
  return s * 1000000000 + uns / (1000000000 / 1000000000);
}

struct scm *
get_internal_run_time ()
{
  struct timespec *ts = __get_internal_run_time_ts;
  clock_gettime (2, ts);
  long time = seconds_and_nanoseconds_to_long (ts->tv_sec - g_start_time->tv_sec,
                                               ts->tv_nsec - g_start_time->tv_nsec);
  return make_number (time);
}

struct scm *
getcwd_ ()
{
  return make_string0 (getcwd (0, 512));
}

struct scm *
dup_ (struct scm *port)
{
  return make_number (dup (port->value));
}

struct scm *
dup2_ (struct scm *old, struct scm *new)
{
  dup2 (old->value, new->value);
  return cell_unspecified;
}

struct scm *
delete_file (struct scm *file_name)
{
  unlink (cell_bytes (file_name->string));
  return cell_unspecified;
}

struct scm *
uname_ ()
{
  uname (__uts);
  struct scm *v = make_vector_ (5, cell_unspecified);
  vector_set_x_ (v, 0, make_string0 (__uts->sysname));
  vector_set_x_ (v, 1, make_string0 (__uts->nodename));
  vector_set_x_ (v, 2, make_string0 (__uts->release));
  vector_set_x_ (v, 3, make_string0 (__uts->version));
  vector_set_x_ (v, 4, make_string0 (__uts->machine));
  return v;
}






extern int errno;
struct __dirstream
{
  int fd;

  char *data;
  size_t allocation;
  size_t size;
  size_t offset;

  off_t filepos;
};

typedef struct __dirstream DIR;







typedef double max_align_t;

int __getdirentries (int filedes, char *buffer, size_t nbytes, off_t * basep);


struct dirent
{
  ino_t d_ino;




  off_t d_off;




  unsigned short int d_reclen;

  unsigned char d_type;

  char d_name[256];
};



DIR *opendir (char const *name);



int closedir (DIR * dirp);





struct dirent *readdir (DIR * dirp);


extern void rewinddir (DIR * dirp);


struct scm *
getpid_ ()
{
  return make_number (getpid ());
}

struct scm *
environ_ (struct scm *args)
{

  if (args == cell_nil)
    {
      char **p = environ;
      struct scm *acc = cell_nil;
      while (*p != 0)
        {
          acc = cons (make_string0 (*p), acc);
          p++;
        }
      return reverse_x_ (acc, cell_nil);
    }

  else if (args->cdr == cell_nil)
    {




      struct scm *env;




      env = args->car;
      while (env->type == 7)
        {
          if (env->car->type != 11)
            error (cell_symbol_wrong_type_arg,
                   cons (args, cstring_to_symbol ("environ")));
          env = env->cdr;
        }
      if (env != cell_nil)
        error (cell_symbol_wrong_type_arg,
               cons (args, cstring_to_symbol ("environ")));



      char **p = environ;
      env = args->car;
      while (env->type == 7)
        {
          *p = malloc (env->car->length + 1);
          strcpy (*p, cell_bytes (env->car->string));
          p++;
          env = env->cdr;
        }
      *p = 0;
      return cell_unspecified;
    }

  else
    {
      error (cell_symbol_wrong_number_of_args,
             cons (args, cstring_to_symbol ("environ")));
    }
}

struct scm *
opendir_ (struct scm *file_name)
{
  struct scm *result;

  if (file_name->type != 11)
    error (cell_symbol_wrong_type_arg,
           cons (file_name, cstring_to_symbol ("opendir")));

  DIR *dirstream = opendir (cell_bytes (file_name->string));

  if (dirstream == 0)
    error (cell_symbol_system_error,
           cons (make_string0 ("Could not open directory"), file_name));



  result = make_bytes (sizeof (DIR *));
  char *p = cell_bytes (result);
  memcpy (p, (char *) &dirstream, sizeof (DIR *));

  return result;
}

struct scm *
closedir_ (struct scm *dir)
{
  if (dir->type != 1 || dir->length != sizeof (DIR *))
    error (cell_symbol_wrong_type_arg,
           cons (dir, cstring_to_symbol ("closedir")));

  char *bytes = cell_bytes (dir);
  DIR *dirstream = *((DIR **) bytes);

  int result = closedir (dirstream);

  if (result != 0)
    error (cell_symbol_system_error,
           cons (make_string0 ("Error closing directory"), dir));

  for (long i = 0; i < dir->length; i++)
      bytes[i] = 0;

  return cell_unspecified;
}

struct scm *
readdir_ (struct scm *dir)
{
  if (dir->type != 1 || dir->length != sizeof (DIR *))
    error (cell_symbol_wrong_type_arg,
           cons (dir, cstring_to_symbol ("readdir")));

  char *bytes = cell_bytes (dir);
  DIR *dirstream = *((DIR **) bytes);

  errno = 0;
  struct dirent *dent = readdir (dirstream);

  if (errno != 0)
    error (cell_symbol_system_error,
           cons (make_string0 ("Error reading from directory"), dir));

  if (dent != 0)
    return make_string0 (dent->d_name);
  else
    return cell_f;
}

struct scm *
pipe_ ()
{
  int fds[2];
  if (pipe (fds) != 0)
    error (cell_symbol_system_error, make_string0 ("Could not create pipe"));
  return cons (make_number (fds[0]), make_number (fds[1]));
}

struct scm *
close_port (struct scm *port)
{
  if (port->type == 6)
    if (close (port->value) != 0)
      error (cell_symbol_system_error, cons (make_string0 ("Error closing port"), port));
}

struct scm *
seek (struct scm *port, struct scm *offset, struct scm *whence)
{
  if (port->type != 6)
    error (cell_symbol_wrong_type_arg,
           cons (port, cstring_to_symbol ("seek")));
  off_t result = lseek (port->value, offset->value, whence->value);
  if (result == -1)
    error (cell_symbol_system_error,
           cons (make_string0 ("Error seeking"), port));
  return make_number (result);
}

struct scm *
chdir_ (struct scm *file_name)
{
  if (file_name->type != 11)
    error (cell_symbol_wrong_type_arg,
           cons (file_name, cstring_to_symbol ("chdir_")));

  if (chdir (cell_bytes (file_name->string)) != 0)
    error (cell_symbol_system_error,
           cons (make_string0 ("Could not change directory"), file_name));

  return cell_unspecified;
}

struct scm *
statbuf_to_vector (struct stat *buf)
{
  struct scm *v = make_vector_ (18, cell_unspecified);
  vector_set_x_ (v, 0, make_number (buf->st_dev));
  vector_set_x_ (v, 1, make_number (buf->st_ino));
  vector_set_x_ (v, 2, make_number (buf->st_mode));
  vector_set_x_ (v, 3, make_number (buf->st_nlink));
  vector_set_x_ (v, 4, make_number (buf->st_uid));
  vector_set_x_ (v, 5, make_number (buf->st_gid));
  vector_set_x_ (v, 6, make_number (buf->st_rdev));
  vector_set_x_ (v, 7, make_number (buf->st_size));
  vector_set_x_ (v, 8, make_number (buf->st_atime));
  vector_set_x_ (v, 9, make_number (buf->st_mtime));
  vector_set_x_ (v, 10, make_number (buf->st_ctime));

  if ((((buf->st_mode) & 0170000) == 0040000))
    vector_set_x_ (v, 13, cstring_to_symbol ("directory"));
  else if ((((buf->st_mode) & 0170000) == 0100000))
    vector_set_x_ (v, 13, cstring_to_symbol ("regular"));
  else if ((((buf->st_mode) & 0170000) == 0010000))
    vector_set_x_ (v, 13, cstring_to_symbol ("fifo"));
  else
    vector_set_x_ (v, 13, cstring_to_symbol ("unknown"));

  vector_set_x_ (v, 14, make_number ((~0170000) & buf->st_mode));





  vector_set_x_ (v, 15, make_number (0));
  vector_set_x_ (v, 16, make_number (0));
  vector_set_x_ (v, 17, make_number (0));

  return v;
}

struct scm *
stat__ (char const *variant, struct scm *args)
{
  int result;
  struct stat buf;
  struct scm *file_name;
  struct scm *exception_on_error = cell_t;

  if (args->type != 7)
    error (cell_symbol_wrong_number_of_args,
           cstring_to_symbol (variant));

  file_name = args->car;

  if (file_name->type != 11)
    error (cell_symbol_wrong_type_arg,
           cons (file_name, cstring_to_symbol (variant)));

  if (args->cdr->type == 7)
    {
      exception_on_error = args->cdr->car;
      if (args->cdr->cdr != cell_nil)
        error (cell_symbol_wrong_number_of_args,
               cstring_to_symbol (variant));
    }


  if (variant[0] == 'l')
    result = lstat (cell_bytes (file_name->string), &buf);
  else
    result = stat (cell_bytes (file_name->string), &buf);

  if (result != 0)
    {
      if (exception_on_error != cell_f)
        error (cell_symbol_system_error,
               cons (make_string0 ("Could not read file attributes"),
                     file_name));
      else
        return cell_f;
    }

  return statbuf_to_vector (&buf);
}

struct scm *
stat_ (struct scm *args)
{
  return stat__ ("stat", args);
}

struct scm *
lstat_ (struct scm *args)
{
  return stat__ ("lstat", args);
}

struct scm *
rename_file (struct scm *old_name, struct scm *new_name)
{
  int result;

  if (old_name->type != 11)
    error (cell_symbol_wrong_type_arg,
           cons (old_name, cstring_to_symbol ("rename-file")));

  if (new_name->type != 11)
    error (cell_symbol_wrong_type_arg,
           cons (new_name, cstring_to_symbol ("rename-file")));

  result = rename (cell_bytes (old_name->string),
                   cell_bytes (new_name->string));

  if (result != 0)
    error (cell_symbol_system_error,
           cons (make_string0 ("Could not rename file"),
                 cons (old_name, new_name)));

  return cell_unspecified;
}


struct scm *
mkdir_ (struct scm *file_name)
{
  if (file_name->type != 11)
    error (cell_symbol_wrong_type_arg,
           cons (file_name, cstring_to_symbol ("mkdir")));

  if (mkdir (cell_bytes (file_name->string), 0777) != 0)
    error (cell_symbol_system_error,
           cons (make_string0 ("Could not make directory"),
                 file_name));

  return cell_unspecified;
}

struct scm *
rmdir_ (struct scm *file_name)
{
  if (file_name->type != 11)
    error (cell_symbol_wrong_type_arg,
           cons (file_name, cstring_to_symbol ("rmdir")));

  if (rmdir (cell_bytes (file_name->string)) != 0)
    error (cell_symbol_system_error,
           cons (make_string0 ("Could not remove directory"),
                 file_name));

  return cell_unspecified;
}

struct scm *
link_ (struct scm *old_name, struct scm *new_name)
{
  if (old_name->type != 11)
    error (cell_symbol_wrong_type_arg,
           cons (old_name, cstring_to_symbol ("link")));
  if (new_name->type != 11)
    error (cell_symbol_wrong_type_arg,
           cons (new_name, cstring_to_symbol ("link")));
  if (link (cell_bytes (old_name->string),
            cell_bytes (new_name->string)) != 0)
    error (cell_symbol_system_error,
           cons (make_string0 ("Could not create (hard) link"),
                 new_name));
  return cell_unspecified;
}

struct scm *
symlink_ (struct scm *old_name, struct scm *new_name)
{
  if (old_name->type != 11)
    error (cell_symbol_wrong_type_arg,
           cons (old_name, cstring_to_symbol ("symlink")));
  if (new_name->type != 11)
    error (cell_symbol_wrong_type_arg,
           cons (new_name, cstring_to_symbol ("symlink")));
  if (symlink (cell_bytes (old_name->string),
               cell_bytes (new_name->string)) != 0)
    error (cell_symbol_system_error,
           cons (make_string0 ("Could not create symbolic link"),
                 new_name));
  return cell_unspecified;
}

struct scm *
umask_ (struct scm *args)
{
  mode_t mask;
  if (args->type == 7)
    {
      if (args->car->type != 6)
        error (cell_symbol_wrong_type_arg,
               cons (args->car, cstring_to_symbol ("umask")));
      if (args->cdr != cell_nil)
        error (cell_symbol_wrong_number_of_args,
               cstring_to_symbol ("umask"));
      return make_number (umask (args->car->value));
    }
  else
    {
      mask = umask (0);
      umask (mask);
      return make_number (mask);
    }
}

struct scm *
utime_ (struct scm *file_name, struct scm *actime, struct scm *modtime)
{
  struct timespec times[2];

  if (file_name->type != 11)
    error (cell_symbol_wrong_type_arg,
           cons (file_name, cstring_to_symbol ("utime")));
  if (actime->type != 6)
    error (cell_symbol_wrong_type_arg,
           cons (actime, cstring_to_symbol ("utime")));
  if (modtime->type != 6)
    error (cell_symbol_wrong_type_arg,
           cons (modtime, cstring_to_symbol ("utime")));

  times[0].tv_sec = actime->value;
  times[0].tv_nsec = 0;
  times[1].tv_sec = modtime->value;
  times[1].tv_nsec = 0;

  if (utimensat (-100, cell_bytes (file_name->string), times, 0) != 0)
    error (cell_symbol_system_error,
           cons (make_string0 ("Could not update timestamps"),
                 file_name));

  return cell_unspecified;
}

struct scm *
sleep_ (struct scm *seconds)
{
  if (seconds->type != 6)
    error (cell_symbol_wrong_type_arg,
           cons (seconds, cstring_to_symbol ("sleep")));
  struct timespec requested_time;
  struct timespec remaining;
  requested_time.tv_sec = seconds->value;
  requested_time.tv_nsec = 0;
  if (nanosleep (&requested_time, &remaining) != 0)
    return make_number (remaining.tv_sec);
  else
    return make_number (0);
}
