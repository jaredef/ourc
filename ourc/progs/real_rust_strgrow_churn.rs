fn main(){
    let mut s=String::new(); let mut i=0;
    while i<20000 { s.push_str("0123456789"); i=i+1; }
    println!("{}", s.len());
}
