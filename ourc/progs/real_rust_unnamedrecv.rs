// The methods that are the identity in a handle-based value model are the identity whatever the receiver
// is -- including a receiver lowering cannot name. Without one list, each such call minted a name like
// $as_str or $len that nothing defined. A typed array names itself arr$str, not arr, so its len needed
// the same widening.
struct T { a:i32 }
fn total(vs:&Vec<Vec<T>>)->i32{
    let hv=&vs[0]; let mut k=0; let mut s=0; while k<hv.len(){ s=s+hv[k].a; k=k+1; } s }
fn main(){
    let mut vs:Vec<Vec<T>>=Vec::new();
    let mut r:Vec<T>=Vec::new(); r.push(T{a:3}); r.push(T{a:4}); vs.push(r);
    let names=["alpha","beta","gamma"];
    let s=String::from("hi");
    println!("{} {} {} {}", total(&vs), names.len(), s.as_str(), s.clone().len()); }
