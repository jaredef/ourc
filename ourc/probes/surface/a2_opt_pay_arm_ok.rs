fn name(o: Option<i32>) -> String {
    match o {
        Some(v) => v.to_string(),
        None => String::from("none"),
    }
}
fn main() {
    let o = Some(4);
    let x = match o { Some(v) => v, None => 0 };
    let s = match x { 4 => "four", _ => "other" };
    println!("{} {} {} {}", x, s, name(Some(2)), name(None));
}
