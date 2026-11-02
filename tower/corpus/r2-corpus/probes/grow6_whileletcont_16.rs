fn main() {
    let mut stack = vec![1i64, 2, 3, 4, 5];
    let mut sum = 0;
    while let Some(x) = stack.pop() {
        if x == 3 { continue; }
        sum += x;
    }
    println!("{}", sum);
}
