fn main(){
    let mut a=String::from(""); a.push_str("ab");
    let b=String::from(a.as_str());
    let c=a.clone();
    a.push_str("cd");
    println!("{} {} {}", a, b, c);
}
