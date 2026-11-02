fn first_even(v:&[i64])->Option<i64>{ let x=v.iter().find(|&&n| n%2==0)?; Some(*x*10) }
fn main(){ let v=vec![1i64,3,4,5]; println!("{}", first_even(&v).unwrap_or(-1)); let e:Vec<i64>=vec![1,3,5]; println!("{}", first_even(&e).unwrap_or(-1)); }
