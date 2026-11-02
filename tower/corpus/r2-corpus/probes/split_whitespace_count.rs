// split_whitespace().count() as a method chain -- regression guard for TI3e-j:
// split_whitespace (16 chars) never matched the split (5-char) handler, so it
// fell through and returned the receiver string. Now an inline word-count loop.
fn main() {
    println!("{}", "a b c".split_whitespace().count());
    println!("{}", "  leading   and   trailing  ".split_whitespace().count());
    println!("{}", "".split_whitespace().count());
    println!("{}", "solo".split_whitespace().count());
    let s = String::from("alpha beta gamma delta");
    println!("{}", s.split_whitespace().count());
}
