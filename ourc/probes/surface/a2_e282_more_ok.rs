fn main() {
    let n: i32 = "42".parse().unwrap();
    let m = "7".parse::<i32>().unwrap();
    let v: Vec<i32> = Vec::new();
    for x in v { println!("{}", x); }
    let e: [i32; 0] = [];
    println!("{} {} {:?}", n, m, e);
}
