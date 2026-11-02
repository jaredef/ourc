fn main(){ let c = if let 1..=9 = 7 { 4 } else { 5 }; let d = if let -10i8..-3i8 = -3i8 { 1 } else { 2 }; println!("{} {}", c, d); }
