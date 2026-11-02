extern "C" fn twice(x: i32) -> i32 { x * 2 }
unsafe extern "C" fn thrice(x: i32) -> i32 { x * 3 }
fn apply(f: extern "C" fn(i32) -> i32, v: i32) -> i32 { f(v) }
fn apply_u(f: unsafe extern "C" fn(i32) -> i32, v: i32) -> i32 { unsafe { f(v) } }
fn plain(f: fn(i32) -> i32, v: i32) -> i32 { f(v) }
fn inc(x: i32) -> i32 { x + 1 }
fn main() {
    println!("{} {} {} {}", twice(4), apply(twice, 5), apply_u(thrice, 2), plain(inc, 9));
}
