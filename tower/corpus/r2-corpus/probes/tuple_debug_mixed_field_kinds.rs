// DBG: {:?} on a tuple with MIXED field kinds must print each field per its type -- &str/String quoted,
// i64/i32 as numbers -- not route every field through the int-only __dbg_els (which printed str pointers).
fn main() {
    let s = String::from("x");
    println!("{:?} {:?} {:?} {:?}", ("hi", 42), (1, "world"), (s, 5), (100i64, 7));
}
