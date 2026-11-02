// C-ARRTOSLICE: a `&[T]` parameter is a VECTOR in our value model, and an array is a different layout, so
// passing `&a` for an array a used to hand the callee an array to read with the vector helpers -- accepted,
// then SEGFAULT. rustc's coercion is array-to-slice; ours is the copy arr_slice already makes.
fn tot(a:&[i64])->i64{ let mut s=0; let mut i=0; while i<a.len() { s=s+a[i]; i=i+1; } s }
fn main(){ let a=[1,2,3]; let v=vec![4,5];
    println!("{} {} {} {}", tot(&a), tot(&a), tot(&a[1..3]), tot(&v)); }
