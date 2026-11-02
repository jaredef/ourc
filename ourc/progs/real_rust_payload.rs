// An enum payload whose type this tier cannot name is UNKNOWN, not int: binding it in a match arm and
// assigning it onward must not be reported as an int-to-enum mismatch.
enum K { A, B }
fn main(){
    let mut vs:Vec<K>=Vec::new(); vs.push(K::B); vs.push(K::A);
    let mut taken=K::A; let mut tmp:Vec<K>=Vec::new(); let mut r=2; let pu=0;
    while r>0 { r=r-1; match vs.pop() { Some(x)=>{ if r==pu { taken=x; } else { tmp.push(x); } }, None=>{} } }
    let mut ns:Vec<i32>=Vec::new(); ns.push(7); let mut t=0;
    match ns.pop() { Some(x)=>{ t=x; }, None=>{} }
    let mut ss:Vec<String>=Vec::new(); ss.push(String::from("hi")); let mut u=String::from("");
    match ss.pop() { Some(x)=>{ u=x; }, None=>{} }
    match taken { K::A=>println!("A {} {} {}", t, u, tmp.len()), K::B=>println!("B {} {} {}", t, u, tmp.len()) } }
