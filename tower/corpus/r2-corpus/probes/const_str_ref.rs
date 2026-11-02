// LAYER3 M3: `const S: &str = ".."`. The int-only const registry didn't store &str consts (printed 0). Now a
// &str const registers as a GK=3 global (GSTART/GCNT hold the literal content span) and emits via
// emit_str_literal at use -- reusing the string machinery; a following .method() routes to dispatch_string.
const GREETING: &str = "hello";
const SEP: &str = "=";
fn main() {
    println!("{}", GREETING);
    println!("{}", GREETING.len());
    let g = GREETING;
    println!("{}", g.starts_with("he"));
    let pair = "key=val";
    if let Some((k, v)) = pair.split_once(SEP) {
        println!("{} {}", k, v);
    }
}
