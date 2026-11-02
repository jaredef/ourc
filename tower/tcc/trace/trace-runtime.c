/* trace-runtime.c -- the shadow-stack runtime for rzir-trace's hooks. Compiled by the NORMAL
 * ccc (never through rzir-trace) so the hooks are not themselves traced. The stack holds the
 * function-name POINTER each `__tenter` was handed (a per-frame local buffer rzir-trace built),
 * so both the error backtrace and the --trace execution log write readable names directly --
 * no external map, no decode step. The name buffers live in the traced frames.
 *
 * Two views over the same hooks:
 *   - __tbacktrace()  -- the live call stack at a fatal point (always on; fired from tcc_error).
 *   - --trace         -- a full, indented execution trace (opt-in; __tverbose(1) enables it, set
 *                        by a --trace scan injected at the top of tcc's main). */
int t_stk[512]; int t_sp; int t_verbose;

int __twrite(int nm) { char *s; int j; s=(char*)nm; j=0; while (s[j] != 0 && j < 128) { j=j+1; } __syscall(1, 2, s, j); return 0; }

int __tenter(int nm) {
  if (t_verbose) { int k; k=0; while (k<t_sp && k<60) { __syscall(1, 2, "  ", 2); k=k+1; } __twrite(nm); __syscall(1, 2, "\n", 1); }
  if (t_sp < 512) { t_stk[t_sp] = nm; } t_sp = t_sp + 1; return 0; }
int __texit() { if (t_sp > 0) { t_sp = t_sp - 1; } return 0; }
int __tdepth() { return t_sp; }
int __treset(int d) { t_sp = d; return 0; }        /* restore depth after a longjmp unwind */
int __tverbose(int v) { t_verbose = v; return 0; } /* --trace: enable the indented execution log */

int __tbacktrace() {
  int i;
  __syscall(1, 2, "  [trace] call stack (innermost first):\n", 40);
  i = t_sp; if (i > 512) { i = 512; }
  while (i > 0) { i = i - 1; __syscall(1, 2, "      ", 6); __twrite(t_stk[i]); __syscall(1, 2, "\n", 1); }
  return 0; }
