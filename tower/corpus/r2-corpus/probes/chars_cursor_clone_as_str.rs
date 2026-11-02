// L4-3 P-CHARS-ITER (cont): the Chars cursor .clone() + .as_str() -- the peek primitive rustc_lexer's Cursor
// uses. Cursor.first()/second() peek by CLONING the Chars and pulling next() off the clone, leaving the real
// position untouched. .clone() copies the [buf,len,pos] cell (advancing the clone does not move the original);
// .as_str() returns the remaining &str [buf+pos, len-pos] from the cursor position.
struct Cursor {
    // model rustc_lexer's cursor: hold the remaining string, peek via a cloned iterator
    rest: &'static str,
}
fn first(c: &Cursor) -> i32 {
    let mut clone = c.rest.chars();
    match clone.next() {
        Some(ch) => ch as i32,
        None => 0,
    }
}
fn second(c: &Cursor) -> i32 {
    let mut clone = c.rest.chars();
    clone.next();
    match clone.next() {
        Some(ch) => ch as i32,
        None => 0,
    }
}
fn main() {
    let s = "abcd";
    // peek-via-clone: the clone advances, the original does not
    let mut it = s.chars();
    let mut peek = it.clone();
    let p = peek.next().unwrap() as u32;
    let real = it.next().unwrap() as u32;
    println!("{} {}", p, real);
    // as_str after two advances
    let mut it2 = s.chars();
    it2.next();
    it2.next();
    println!("{}", it2.as_str());
    // cursor-model peeks
    let c = Cursor { rest: "xy" };
    println!("{} {}", first(&c), second(&c));
}
