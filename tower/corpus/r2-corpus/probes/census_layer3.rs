// LAYER3 census crate: macro/closure/const-heavy program exercising the Layer-3 subset TOGETHER --
// macro_rules repetition (fan-out + expr-accum), closures (-> T block, fn-pointer, iter-adapter), const &str/int,
// alongside the Layer-1/2 baseline (Vec/BTreeMap/Option/String/iterators/format/patterns). Proof-of-work that the
// Layer-3 subset is coherent: compiles == rustc byte-for-byte. (Const-ARRAY .len() is an orthogonal Layer-2 gap,
// recorded; avoided here.)
use std::collections::BTreeMap;

const TAG: &str = "score";
const BASE: i32 = 100;

macro_rules! sum {
    ($($x:expr),*) => { { let mut s = 0; $( s += $x; )* s } };
}
macro_rules! report {
    ($($x:expr),*) => { $( println!("{} = {}", TAG, $x); )* };
}

fn weight(i: i32) -> i32 {
    match i {
        0 => 3,
        1 => 2,
        _ => 1,
    }
}

fn weighted(vals: &Vec<i32>) -> i32 {
    let mut total = 0;
    for (i, v) in vals.iter().enumerate() {
        total += v * weight(i as i32);
    }
    total
}

fn classify(n: i32) -> &'static str {
    if n < 0 { "neg" } else if n == 0 { "zero" } else { "pos" }
}

fn main() {
    let scale: fn(i32) -> i32 = |x| -> i32 { x * 10 };
    let vals = vec![3, 1, 4, 1, 5];
    let squares: i32 = vals.iter().map(|x| -> i32 { x * x }).sum();
    let evens: Vec<i32> = vals.iter().filter(|x| **x % 2 == 0).map(|x| scale(*x)).collect();

    report!(squares, weighted(&vals));
    let total = sum!(BASE, 20, 30, weighted(&vals));

    println!("{} len {}", TAG, TAG.len());
    println!("squares={} total={} evens={:?}", squares, total, evens);

    let mut counts: BTreeMap<i32, i32> = BTreeMap::new();
    for &v in vals.iter() {
        let e = counts.entry(v).or_insert(0);
        *e += 1;
    }
    for (k, c) in &counts {
        println!("{}: {} ({})", k, c, classify(*k - 2));
    }

    let line = "name=cruft";
    if let Some((k, v)) = line.split_once("=") {
        println!("kv {} -> {}", k, v);
    }
}
