// DBG: {:?} on a Vec<(..)> (tuple elements) must print [(a, b), ..], not element pointers. The element
// interned Ty (TYTAG==11 tuple) is now captured (AVELTY) so the Vec-Debug printer takes a tuple branch.
fn main() {
    let lits: Vec<(i32, i32)> = vec![(1, 2), (3, 4)];
    let v = vec![10, 20, 30];
    let pairs: Vec<(usize, i32)> = v.iter().enumerate().map(|(i, x)| (i, *x)).collect();
    println!("{:?} {:?}", lits, pairs);
}
