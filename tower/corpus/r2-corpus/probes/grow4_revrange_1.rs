fn main(){ let mut out=String::new(); for i in (0..5).rev(){ out.push((b'0'+i as u8) as char); } println!("{}",out); }
