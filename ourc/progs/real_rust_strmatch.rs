fn kind(s: &str) -> i64 {
    match s {
        "cat" => 1,
        "dog" => 2,
        _ => 0,
    }
}
fn main() {
    println!("{} {} {}", kind("cat"), kind("dog"), kind("fox"));
}
