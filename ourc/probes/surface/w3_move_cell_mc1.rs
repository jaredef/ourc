fn main(){ let mut c = 10; let mut inc = move || { c += 1; c }; let a = inc(); let b = inc(); c += 100; println!("{} {} {} {}", a, b, inc(), c); }
