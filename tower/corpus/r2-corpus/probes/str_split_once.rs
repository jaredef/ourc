// LAYER2: str::split_once(sep) -> Option<(&str,&str)>. __str_find locates the first sep; Some((left,right))
// as byte-offset &str slices, None when absent. The (&str,&str) tuple type is registered so the Some((k,v))
// binder (R12) restores both fields as &str. Only the FIRST occurrence splits (rest stays in the right half).
fn parse_kv(line: &str) -> String {
    if let Some((k, v)) = line.split_once("=") {
        format!("{}->{}", k, v)
    } else {
        String::from("(no sep)")
    }
}
fn main() {
    println!("{}", parse_kv("name=cruft"));
    println!("{}", parse_kv("path=/a=b"));
    println!("{}", parse_kv("bare"));
    match "hello world foo".split_once(" ") {
        Some((first, rest)) => println!("first={} rest={}", first, rest),
        None => println!("none"),
    }
}
