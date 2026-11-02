// tractable: flatten() on an iterator of Vecs -> concat inner elements (nested-loop lowering).
fn main() {
    let g: Vec<Vec<i32>> = (0..3).map(|i| vec![i, i + 1]).collect();
    let s: i32 = g.iter().flatten().sum();
    println!("{}", s);                           // (0+1)+(1+2)+(2+3) = 9
    let h: Vec<Vec<i32>> = vec![vec![1, 2], vec![3]];
    let f: Vec<i32> = h.iter().flatten().cloned().collect();
    println!("{} {}", f.len(), f[2]);            // 3 3
}
