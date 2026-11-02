// LAYER2 P3-sub: `&v[..]` / `&v[..hi]` full-range (no-low-bound) slicing on a Vec (and &a[..] via a bound
// local). Was NOELF: the &v[a..b] handler parsed the low bound before checking for `..`, so a leading `..`
// hit parse_expr on the `.`. Now a leading `..` omits the low bound (a=0).
fn main() {
    let v = vec![10, 20, 30, 40];
    let whole = &v[..];
    println!("{}", whole.len());
    let head = &v[..2];
    let s: i32 = head.iter().sum();
    println!("{}", s);
    let tail = &v[1..];
    println!("{:?}", tail);
    let a = [1, 2, 3];
    let av = &a[..];
    println!("{}", av.len());
}
