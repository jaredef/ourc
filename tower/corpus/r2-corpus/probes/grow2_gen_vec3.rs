fn main(){ let v:Vec<Vec<Vec<i32>>>=vec![vec![vec![1,2],vec![3]],vec![vec![4,5,6]]]; let mut s=0; for a in &v { for b in a { for c in b { s+=c; } } } println!("{}", s); }
