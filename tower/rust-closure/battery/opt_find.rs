fn first_even(v: &Vec<i32>) -> Option<i32> {
    let mut i = 0;
    while i < v.len() {
        if v[i] % 2 == 0 { return Some(v[i]); }
        i = i + 1;
    }
    None
}
fn main() {
    let a = vec![1, 3, 5, 8, 9];
    let b = vec![1, 3, 5, 7, 9];
    match first_even(&a) { Some(x) => println!("first even {}", x), None => println!("none") }
    match first_even(&b) { Some(x) => println!("first even {}", x), None => println!("none") }
    println!("a: {}", first_even(&a).unwrap());
    println!("b or -1: {}", first_even(&b).unwrap_or(-1));
    println!("b is_none: {}", first_even(&b).is_none() as i32);
}
