fn mm(v:&[i64])->(i64,i64){ if v.len()==1 { return (v[0],v[0]); } let (a,b)=mm(&v[1..]); let h=v[0]; (if h<a{h}else{a}, if h>b{h}else{b}) }
fn main(){ let v=[3,7,1,9,4]; let (lo,hi)=mm(&v); println!("{} {}",lo,hi); }
