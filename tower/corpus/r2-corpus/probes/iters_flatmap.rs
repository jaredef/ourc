fn main(){let v=[1,2,3];let c:i64=v.iter().flat_map(|&x|(0..x)).count() as i64;println!("{}",c);}
