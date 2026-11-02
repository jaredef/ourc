fn cls(n:i64)->i64{ match n { 0..=9=>1, 10..=99=>2, _=>3 } }
fn main(){ println!("{}", cls(5)+cls(50)*10+cls(500)*100); }
