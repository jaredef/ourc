fn fizzbuzz(n: i32) -> &'static str {
    if n % 15 == 0 { "FizzBuzz" } else if n % 3 == 0 { "Fizz" } else if n % 5 == 0 { "Buzz" } else { "" }
}
fn main() {
    let mut i = 1;
    while i <= 15 {
        let s = fizzbuzz(i);
        if s.len() == 0 { println!("{}", i); } else { println!("{}", s); }
        i = i + 1;
    }
}
