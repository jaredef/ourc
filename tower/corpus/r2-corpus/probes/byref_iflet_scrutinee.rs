// D004-CORE IC-1: `if let Some(x) = &opt` (statement + expression position) -- the transparent
// leading `&` must be skipped so the scrutinee is the cell pointer, not the slot address.
fn main() {
    let o: Option<i32> = Some(41);
    let mut acc = 0;
    if let Some(x) = &o { acc = acc + *x; }
    let y = if let Some(x) = &o { *x + 1 } else { 0 };
    println!("{} {}", acc, y);
}
