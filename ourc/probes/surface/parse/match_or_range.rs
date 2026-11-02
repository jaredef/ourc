fn main() { for n in 0..6 { let s = match n { 0 | 1 => "lo", 2..=3 => "mid", _ => "hi" }; println!("{}", s); } }
