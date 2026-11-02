fn main() {
    let tokens = ["3", "4", "+", "5", "*"];
    let mut stack: Vec<i64> = Vec::new();
    for t in tokens.iter() {
        if *t == "+" {
            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();
            stack.push(a + b);
        } else if *t == "*" {
            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();
            stack.push(a * b);
        } else {
            let n: i64 = t.parse().unwrap();
            stack.push(n);
        }
    }
    println!("{}", stack.pop().unwrap());
}
