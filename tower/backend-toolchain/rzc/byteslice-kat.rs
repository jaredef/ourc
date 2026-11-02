/* K2: byte-slice reads must work -- the old ecopy double-def corrupted descriptor pointers,
   so b"..."[i]/.len() and str.as_bytes()[i] silently returned 0. Returns 167 = 47+116+4. */
fn main() -> i32 {
    unsafe {
        let bl = b"/tmp";
        let s = "/tmp";
        let sb = s.as_bytes();
        return bl[0] + bl[1] + sb.len() + (bl.len() - sb.len());   /* 47 + 116 + 4 + 0 */
    }
}
