fn main() {
    let mut done = vec![false; 4];
    done[1] = true;
    done[3] = true;
    let mut c = 0;
    for v in 0..4 { if !done[v] { c += 1; } }
    let d = vec![true, false, true];
    println!("{} {} {} {}", c, d[0], !d[0], done[1]);
}
