struct S(i64, i64, i64);
fn main(){
let x=(1,2,3);
match x { (a,b,..) => println!("A {} {}",a,b) }
match x { (a,b,c,..) => println!("D {} {} {}",a,b,c) }
let s=S(1,2,3);
}
