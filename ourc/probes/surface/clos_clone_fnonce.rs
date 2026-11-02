fn call<T, F: FnOnce() -> T>(f: F) -> T { f() }
fn main(){ let a = 5; let hello = || a + 1; let h2 = hello.clone(); println!("{} {} {}", call(hello.clone()), call(h2), call(hello)); }
