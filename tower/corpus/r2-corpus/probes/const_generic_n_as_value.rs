// P-MONO-CONST-GENERIC: a const-generic length `N` used as a VALUE in the body (return N, while i<N, N*2)
// must resolve to the concrete array length. Was unbound (ret v-1 / garbage); now bound to the array
// descriptor's len field. `a.iter()`/`a.len()` forms already worked.
fn n_of<const N: usize>(_a: [i32; N]) -> usize { N }
fn arr_sum<const N: usize>(a: [i32; N]) -> i32 {
    let mut s = 0; let mut i = 0;
    while i < N { s += a[i]; i += 1; }
    s
}
fn main() {
    println!("{} {} {}", n_of([10, 20, 30]), arr_sum([10, 20, 30]), arr_sum([1, 2, 3, 4]));
}
