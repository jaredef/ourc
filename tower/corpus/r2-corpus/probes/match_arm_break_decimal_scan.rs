fn cls() -> i32 { let b = "123x".as_bytes(); let mut i = 0; loop { let c = b[i as usize] as char; match c { '_' => { i += 1; } '0'..='9' => { i += 1; } _ => break, } } i }
fn main(){ println!("{}", cls()); }
