// A closure's DECLARED parameter type was read and discarded at lifting, so every lifted closure
// parameter carried an empty recorded type -- and a body calling o.push on one had an unnameable
// receiver, which reached the emitter as an undefined call. Function parameters have always kept their
// declared type; closure parameters are the second place the same rule lives.
fn main(){
    let mut o:Vec<String>=Vec::new();
    let mut add=|t:&str, o:&mut Vec<String>| { if t.len()>0 { o.push(String::from(t)); } };
    add("a",&mut o); add("",&mut o); add("bb",&mut o);
    let mut v:Vec<i32>=Vec::new();
    { let mut p=|x:i32, v:&mut Vec<i32>| { v.push(x*2); }; p(3,&mut v); p(4,&mut v); }
    let f=|s:&str| -> usize { s.len() };
    let g=|a,b| a+b;
    println!("{} {} {} {} {} {}", o.len(), o[1], v.len(), v[1], f("abcd"), g(2,3)); }
