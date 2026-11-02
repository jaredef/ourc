// R2a P4 guard: a `let` whose type closes two generics at once (`Vec<Vec<T>>`, lexed as ONE `>>`) WITH an initializer is not a
// deferred `let x;` -- it was, and the definite-assignment walk refused it as possibly-uninitialized (E0381).
fn main() {
    let mut lets: Vec<Vec<i64>> = Vec::new();
    lets.push(vec![1, 2]);
    let c = lets.len() > 0;
    if c { lets.push(vec![3]); }
    let vs: Vec<Vec<i64>> = vec![vec![4]];
    println!("{} {} {}", lets.len(), lets[0][1], vs[0][0]);
}
