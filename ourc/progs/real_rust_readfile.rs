// Our own driver reads its input with fs::read_to_string, so a compiler meant to compile that driver needs
// the primitive. It is a runtime helper over libc answering our str, plus a second helper that reports
// whether the file opens at all -- so the prelude wrapper can answer a real Result rather than claiming
// success on an unreadable path.
fn main(){
    let t=match std::fs::read_to_string("/etc/hostname") { Ok(x)=>x, Err(_e)=>String::from("") };
    let miss=match std::fs::read_to_string("/nonexistent/zz") { Ok(_x)=>1, Err(_e)=>0 };
    println!("{} {}", t.len()>0, miss); }
