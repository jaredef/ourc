fn main() {
    let mut stack = vec![vec![1,2], vec![3]];
    let mut t = 0;
    while let Some(mut inner) = stack.pop() {
        while let Some(x) = inner.pop() { t += x; }
    }
    println!("{}", t);
}
