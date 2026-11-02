// DBG: {:?} on an Option<&str>/Option<String> must quote the payload -- Some("x") not Some(<pointer>).
// emit_opt_debug was called with pk=0 (int); now the Option payload kind (EOPTK 1=&str/3=String) is captured
// (AOPTK) and threaded, and first()/last()/find() set EOPTK from the Vec's element Ty.
fn main() {
    let v = vec!["x", "y", "z"];
    let e: Vec<&str> = vec![];
    let s: Option<&str> = Some("hi");
    let t: Option<String> = Some(String::from("wo"));
    println!("{:?} {:?} {:?} {:?} {:?}", v.first(), v.last(), e.first(), s, t);
}
