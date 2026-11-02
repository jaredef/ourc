fn main() {
    let mut n = 0;
    'outer: while break 'outer {}
    'scan: loop {
        n += 1;
        if n > 3 {
            break 'scan;
        }
    }
    let v = 'pick: loop {
        break 'pick n * 2;
    };
    println!("{} {}", n, v);
}
