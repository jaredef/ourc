// An unbound name is UNKNOWN, not invalid: a slice of a local the argument-type walk cannot see must not
// be typed Vec. env_get answers -1 for a miss and the slice rule guarded only the -3 sentinel.
fn t(s:&str)->bool{ s.len()>0 }
fn main(){
    let v=vec![String::from("ab$cd"), String::from("x")];
    let mut i=0; let mut hits=0;
    while i<v.len(){
        let f=&v[i];
        if f.len()>2 { let o=String::from(&f[0..2]); if t(&o) { hits=hits+1; } println!("{}", o); }
        i=i+1; }
    println!("{}", hits); }
