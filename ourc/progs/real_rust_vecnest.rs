// A Vec's tag is 1000 plus its element's, so a Vec of a Vec minted the typed-ARRAY range and indexing one
// stripped two levels at once. A nested Vec now answers the untyped Vec tag -- still a Vec, element
// unknown -- and indexing an untyped Vec yields an unknown element rather than an invented int.
struct T { a:i32 }
fn main(){
    let mut vs:Vec<Vec<T>>=Vec::new();
    let mut r:Vec<T>=Vec::new(); r.push(T{a:3}); r.push(T{a:4}); vs.push(r);
    let hv=&vs[0]; let mut k=0; let mut s=0; while k<hv.len(){ s=s+hv[k].a; k=k+1; }
    let mut ns:Vec<Vec<i32>>=Vec::new(); ns.push(vec![1,2,3]); ns.push(vec![4]);
    let mut t=0; let mut i=0; while i<ns.len(){ let mut q=0; while q<ns[i].len(){ t=t+ns[i][q]; q=q+1; } i=i+1; }
    let v=vec![9,8]; let a=[1,2,3];
    println!("{} {} {} {}", s, t, v[1], a[2]); }
