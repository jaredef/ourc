// DBG: {:?} on a Vec<&str>/Vec<String> must print ["a", "b", ..] (quoted), not element descriptor pointers.
// The Vec-Debug printer had int/struct/tuple branches; a str-element branch (AVELTY TYTAG 3/4) was missing.
fn main() {
    let a: Vec<&str> = vec!["x", "y", "z"];
    let b: Vec<String> = vec![String::from("p"), String::from("q")];
    let c: Vec<&str> = "a,b,c".split(",").collect();
    println!("{:?} {:?} {:?}", a, b, c);
}
