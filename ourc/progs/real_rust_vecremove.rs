// Vec::remove shifts the tail down and answers the element it took out. It needs no new runtime
// primitive -- get, set and the prelude's own pop are enough -- so it is a prelude function, like max and
// position. Its RESULT type is the receiver's element type, exactly as get's is: taking the prelude
// signature's declared int literally printed a String's handle as a number.
fn main(){
    let mut v=vec![10,20,30,40]; let x=v.remove(1);
    let mut w:Vec<String>=Vec::new();
    w.push(String::from("a")); w.push(String::from("b")); w.push(String::from("c"));
    let y=w.remove(0);
    let mut u=vec![1,2,3,4,5]; let a=u.remove(0); let b=u.remove(3);
    // and as a STATEMENT, whose value is discarded -- inside an if without an else
    let mut t:Vec<String>=Vec::new(); t.push(String::from("p")); t.push(String::from("q"));
    if t.len()>1 { t.remove(0); }
    println!("{} {} {} {} | {} {} {} | {} {} {} {} | {} {}", x, v.len(), v[0], v[2], y, w.len(), w[0], a, b, u.len(), u[2], t.len(), t[0]); }
