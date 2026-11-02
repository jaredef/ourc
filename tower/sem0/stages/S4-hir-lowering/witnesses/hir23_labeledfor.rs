fn f(v: &[i32]) -> i32 {
    let mut s = 0;
    'a: for x in v {
        s += *x;
        if s > 9 { break 'a; }
    }
    s
}

fn main() {}
