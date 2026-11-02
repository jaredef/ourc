fn main(){ let v=[10,20,30,40]; if let [first, rest @ ..]=v { let s:i64=rest.iter().sum(); println!("{}",first+s); } else { println!("0"); } }
