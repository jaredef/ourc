import ctypes, os, sys, signal
libc = ctypes.CDLL("libc.so.6", use_errno=True)
c_long=ctypes.c_long; c_void_p=ctypes.c_void_p
libc.ptrace.restype=c_long
libc.ptrace.argtypes=[c_long,c_long,c_void_p,c_void_p]
PTRACE_TRACEME=0; PTRACE_CONT=7; PTRACE_GETREGS=12; PTRACE_KILL=8
class regs(ctypes.Structure):
    _fields_=[(n,ctypes.c_ulonglong) for n in
      ("r15","r14","r13","r12","rbp","rbx","r11","r10","r9","r8","rax","rcx","rdx","rsi","rdi","orig_rax","rip","cs","eflags","rsp","ss","fs_base","gs_base","ds","es","fs","gs")]
prog=sys.argv[1]
pid=os.fork()
if pid==0:
    r=libc.ptrace(PTRACE_TRACEME,0,None,None)
    fd=os.open("/dev/null",os.O_RDONLY); os.dup2(fd,0)
    fdo=os.open("/dev/null",os.O_WRONLY); os.dup2(fdo,1); os.dup2(fdo,2)
    os.execv(prog,[prog]); os._exit(127)
first=True
while True:
    _,st=os.waitpid(pid,0)
    if os.WIFEXITED(st): print("exited",os.WEXITSTATUS(st)); break
    if os.WIFSIGNALED(st): print("killed by",os.WTERMSIG(st)); break
    if os.WIFSTOPPED(st):
        sig=os.WSTOPSIG(st)
        if first:
            first=False; libc.ptrace(PTRACE_CONT,pid,None,None); continue
        if sig==signal.SIGSEGV:
            r=regs(); libc.ptrace(PTRACE_GETREGS,pid,None,ctypes.byref(r))
            print("SIGSEGV rip=0x%x rax=0x%x rbx=0x%x rcx=0x%x rdx=0x%x rsi=0x%x rdi=0x%x rbp=0x%x rsp=0x%x"%(r.rip,r.rax,r.rbx,r.rcx,r.rdx,r.rsi,r.rdi,r.rbp,r.rsp))
            libc.ptrace(PTRACE_KILL,pid,None,None); break
        else:
            libc.ptrace(PTRACE_CONT,pid,None,ctypes.c_void_p(sig)); 
