// LAYER3 (M2 residual): iter-adapter closures with an explicit return type + block body --
// `.map(|x| -> T { .. })` / `.filter(|x| -> bool { .. })`. emit_iter_map/emit_iter_filter now skip the `-> T`
// after the closing `|` (mirrors R1 in parse_closure), and filter's predicate parse is block-aware.
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let sq: i32 = v.iter().map(|x| -> i32 { x * x }).sum();
    println!("{}", sq);
    let evens: Vec<i32> = v.iter().filter(|x| -> bool { **x % 2 == 0 }).map(|x| x + 0).collect();
    println!("{:?}", evens);
    let stepped: i32 = v
        .iter()
        .map(|x| -> i32 {
            let y = x + 1;
            y * 2
        })
        .sum();
    println!("{}", stepped);
}
