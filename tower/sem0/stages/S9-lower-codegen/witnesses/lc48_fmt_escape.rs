fn main() {
    println!("a{{b}}c");
    let s = format!("x {{{}}} y", 42);
    println!("{}", s);
}
