// C-MEMPLACE: mem::replace is a place operation, and its VALUE is the old one -- which also needed
// C-LETTAIL, because the desugar writes before handing the binding back.
fn main(){ let mut s=String::from("old"); let was=std::mem::replace(&mut s, String::from("new")); println!("{} {}",s,was); }
