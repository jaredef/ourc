static mut ST: [i32; 4] = [0, 0, 0, 0];
fn main() -> i32 {
    unsafe {
        let pid = __syscall(57, 0, 0, 0);          /* fork */
        if pid == 0 {
            __syscall(60, 42, 0, 0);               /* child: exit(42) */
        }
        __syscall(61, pid, &ST[0], 0, 0);          /* wait4(pid, &status, 0, 0) -- needs r10=0 (4th arg) */
        let status = ST[0];
        return (status / 256) % 256;               /* WEXITSTATUS */
    }
}
