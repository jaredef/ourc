fn f(v: &[i32]) -> i32 {
    match v {
        [] => 0,
        [x] => *x,
        [x, rest @ ..] => *x + rest.len() as i32,
    }
}

fn main() {}
