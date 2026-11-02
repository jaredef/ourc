fn main() {
    let a = [1, 2, 3, 4];
    let r = match a { [first, .., last] => first + last };
    println!("{}", r);
}
