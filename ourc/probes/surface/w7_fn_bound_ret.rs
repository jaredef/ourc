fn call_it<F>(f: F) -> String where F: FnOnce(String) -> String { f("Fred".to_string()) }
fn twice<F>(f: F) -> String where F: Fn(i32) -> String { format!("{}{}", f(1), f(2)) }
fn main() {
    let g = "How's life, ".to_string();
    println!("{}", call_it(|s: String| -> String { format!("{}{}", g, s) }));
    println!("{}", twice(|n: i32| -> String { format!("<{}>", n) }));
}
