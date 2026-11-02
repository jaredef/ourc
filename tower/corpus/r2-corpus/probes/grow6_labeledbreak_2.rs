fn main() {
    let mut acc = 0;
    let r = 'outer: loop {
        let mut i = 0;
        loop {
            i += 1;
            acc += i;
            if acc > 20 { break 'outer acc * 2; }
            if i > 5 { break; }
        }
    };
    println!("{}", r);
}
