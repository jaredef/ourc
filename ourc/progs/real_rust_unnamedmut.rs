// The mutators must answer on a receiver lowering cannot name. A str always names itself, so an unnamed
// push or pop is vector-shaped -- and a Vec is a handle, so a push through a &mut parameter is visible to
// the caller. (push_str through a &mut String parameter is NOT: our String is a value, so that stays
// refused -- see the rung-55 record.)
fn fill(out:&mut Vec<i32>, n:i32){ let mut k=0; while k<n { out.push(k*2); k=k+1; } }
fn drain(out:&mut Vec<i32>)->i32{ let mut s=0; loop { match out.pop() { Some(x)=>{ s=s+x; }, None=>{ break; } } } s }
fn names(out:&mut Vec<String>){ out.push(String::from("a")); out.push(String::from("bb")); }
fn main(){
    let mut v:Vec<i32>=Vec::new(); fill(&mut v, 4);
    let n=v.len(); let s=drain(&mut v);
    let mut w:Vec<String>=Vec::new(); names(&mut w);
    println!("{} {} {} {}", n, s, w.len(), w[1]); }
