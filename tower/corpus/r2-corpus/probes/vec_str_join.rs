// R26b: Vec<&str>/Vec<String>.join(SEP) -> a String (elements concatenated with SEP between). Was unhandled
// in dispatch_vec -> returned a pointer.
fn main() {
    let w = vec!["a", "b", "c"];
    let s = vec![String::from("x"), String::from("y")];
    let parts: Vec<&str> = "1,2,3".split(",").collect();
    println!("{} {} {} {}", w.join("-"), s.join(", "), parts.join("|"), vec!["one"].join(","));
}
