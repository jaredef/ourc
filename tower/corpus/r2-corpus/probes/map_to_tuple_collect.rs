// R10/tuple: v.iter().map(|x| (a,b)).collect() must tag the result Vec's element as a tuple Ty, so {:?} and
// for-destructure see tuples not pointers. The plain map path captured EISVEC/EITYP from the body but not
// EISTUP/ETUPTY (only enumerate.map did); now it sets VELTY/DVELTY to the tuple type.
fn main() {
    let v = vec![1, 2, 3];
    let r: Vec<(i32, i32)> = v.iter().map(|x| (*x, *x * 2)).collect();
    let mut acc = 0;
    for (a, b) in &r { acc += a + b; }
    println!("{:?} {}", r, acc);
}
