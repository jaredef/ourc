use std::collections::HashMap;

type Pair = (i32, i32);

struct Wrap<T> { inner: T }

impl<T> Wrap<T> {
    fn new(v: T) -> Wrap<T> { Wrap { inner: v } }
}

fn apply(f: fn(i32) -> i32, x: i32) -> i32 { f(x) }

fn find(v: &[i32], target: i32) -> i32 {
    let mut i = 0;
    while i < v.len() {
        if v[i] == target {
            return i as i32;
        }
        i += 1;
    }
    -1
}

fn transform(v: Vec<i32>) -> i32 {
    let doubled = v.iter().map(|x| x * 2);
    let mut acc = 0;
    for d in doubled {
        acc += d;
    }
    acc
}

fn maybe(o: Option<i32>) -> i32 {
    if let Some(n) = o {
        n
    } else {
        0
    }
}

fn ranges(n: i32) -> i32 {
    let mut s = 0;
    for i in 0..n {
        s += i;
    }
    s
}

fn main() {}
