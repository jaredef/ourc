// tractable: find_map with a `|&x|` ref-pattern param + early `return Some(..)` in a block body.
// Was: parse_closure's rd_ident on `&` read nothing -> x unbound (v-1); find_map passed the elem address.
fn main() {
    let v = vec![1, 5, 3, 9, 2];
    let found = v.iter().find_map(|&x| {
        if x > 4 { return Some(x * 2); }
        None
    });
    println!("{}", found.unwrap_or(0));   // first x>4 is 5 -> 10
    // no-match -> None
    let w = vec![1, 2, 3];
    let g = w.iter().find_map(|&x| if x > 9 { Some(x) } else { None });
    println!("{}", g.unwrap_or(-1));      // -1
}
