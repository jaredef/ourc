fn main(){ let n=7i64; let r=match n { x @ 1..=5 => x*2, x @ 6..=10 => x*3, _=>0 }; println!("{}", r); }
