// R35: a trailing comma in a `vec![a, b, c,]` literal ends the list -- was parsed as a
// spurious extra element (`parse_expr` on the `]` pushed a 0), yielding `[1, 2, 3, 0]`.
// rustfmt emits a trailing comma on every multi-line vec!/array, so this is pervasive.
fn main() {
    let nums = vec![
        1,
        2,
        3,
    ];
    println!("{:?}", nums);

    let words = vec!["a", "b",];
    println!("{:?}", words);

    let sum: i32 = nums.iter().sum();
    println!("{}", sum);
}
