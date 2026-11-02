fn main() {
    let mut s = String::new();
    for i in 0..5 { s.push_str("ab"); }
    println!("{}", s);
    let mut grid = String::new();
    for r in 0..3 {
        for c in 0..3 { grid.push('#'); }
        grid.push('\n');
    }
    print!("{}", grid);
}
