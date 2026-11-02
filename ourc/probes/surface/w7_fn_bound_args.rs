fn call_it<F>(f: F) where F: FnOnce(String) -> String { println!("{}", f("Fred".to_string())) }
fn call_this<F>(f: F) where F: FnOnce(&str) { f("Hello!"); }
fn pair<F>(f: F) -> i32 where F: Fn(i32, i32) -> i32 { f(3, 4) }
fn main() {
    let g = "Hi ".to_string();
    call_it(|s| format!("{}{}", g, s));
    call_this(|s| println!("{}", s));
    println!("{}", pair(|a, b| a * b));
}
