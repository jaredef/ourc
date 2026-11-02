// LAYER2 P3+: nested tuple pattern in a variant payload -- `Some((a, b))` / `match .. Some((a, b, c))`.
// Was: match silently fell through; if-let FE-rejected. Now the payload tuple cell is destructured field
// by field, guarded so a mismatched arm (None) never derefs the payload. Unblocks Option<(&str,&str)>
// consumers (e.g. split_once) later. Scoped to i32 tuple fields.
fn pick(o: Option<(i32, i32)>) -> i32 {
    if let Some((a, b)) = o {
        a * 10 + b
    } else {
        -1
    }
}
fn main() {
    println!("{}", pick(Some((3, 4))));
    println!("{}", pick(None));
    let t = Some((1, 2, 3));
    match t {
        Some((a, b, c)) => println!("{} {} {}", a, b, c),
        None => println!("none"),
    }
    let p = Some((7, 8));
    match p {
        Some((_, b)) => println!("second {}", b),
        None => {}
    }
}
