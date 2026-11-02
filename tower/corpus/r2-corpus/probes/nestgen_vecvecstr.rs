fn main() {
    let g: Vec<Vec<&str>> = vec![vec!["a", "b"], vec!["c"]];
    let n: usize = g.iter().map(|r| r.len()).sum();
    println!("{}", n);
}
