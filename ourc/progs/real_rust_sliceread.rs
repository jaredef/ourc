// A slice in a value position is a COPY in this value model, so a method that WRITES through it would be
// lost -- those stay refused. A method that READS is a different matter and rustc accepts it everywhere,
// so the refusal is narrowed to the mutators.
fn main(){
    let s="abcde";
    let v=vec![1,2,3,4];
    println!("{} {} {} {}", s[1..4].len(), s[1..4].to_string(), v[1..3].len(), (&s[1..4]).len()); }
