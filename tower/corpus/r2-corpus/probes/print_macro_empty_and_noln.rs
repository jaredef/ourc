// print! (no trailing newline) and empty println!() -- regression guard for
// TI3e: empty print!()/println!() has no format string; emit_print must not
// read the trailing source (`); }`) as literal format chars.
fn main() {
    print!("a");
    print!("{}", "b");
    for i in 0..3 { print!("{}", i); }
    println!();
    let w = vec!["xy".to_string(), "z".to_string()];
    for s in &w { print!("{}", s); }
    println!();
    println!("done");
}
