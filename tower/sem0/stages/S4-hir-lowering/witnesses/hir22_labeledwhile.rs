fn f() -> i32 {
    let mut n = 0;
    'a: while n < 10 {
        n += 1;
        if n == 5 { break 'a; }
        continue 'a;
    }
    n
}

fn main() {}
