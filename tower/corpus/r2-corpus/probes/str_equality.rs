// &str / String equality via == and != -- regression guard for TI3e-k (N-str-eq):
// == on string operands fell through to a descriptor-POINTER compare (always
// false for distinct descriptors); now routes to emit_streq_inline (byte compare).
// A high-amortization Tier-B/E node: a non-last node in csv/tokenizer/calc/wordfreq
// cascades, so it flips no existing corpus program alone -- this probe guards it.
fn main() {
    println!("{}", "a" == "a");
    println!("{}", "a" == "b");
    let w = "abc";
    println!("{}", w == "abc");
    println!("{}", w != "xyz");
    let a = "x"; let b = "x";
    println!("{}", a == b);
    let s = String::from("hi");
    println!("{}", s == "hi");
    let mut c = 0;
    for t in ["red", "green", "red"] { if t == "red" { c += 1; } }
    println!("{}", c);
}
