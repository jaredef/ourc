fn main(){
    let mut a=String::from("ab");
    let b=String::from(a.as_str());
    a.push_str("cd");
    let mut c=String::from(b.as_str()); c.push_str("Z");
    println!("{} {} {}", a, b, c);
}
