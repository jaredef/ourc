fn main(){
let s="a,b,,c"; let p:Vec<&str>=s.split(',').filter(|x|!x.is_empty()).collect(); println!("{} {}",p.len(),p.join("-"));
let n=s.split(',').filter(|x| !x.is_empty()).count(); println!("{}",n);
let t="ab cd  e"; let w:Vec<String>=t.split_whitespace().map(|x| x.to_uppercase()).collect(); println!("{:?}",w);
let r:Vec<&str>=s.split(',').rev().skip(1).filter(|x| x.len()>0).collect(); println!("{:?}",r);
}
