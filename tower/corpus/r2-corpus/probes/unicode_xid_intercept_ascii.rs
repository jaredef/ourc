fn is_xid_start(c: char) -> bool { (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') }
fn is_xid_continue(c: char) -> bool { (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9') || c == '_' }
fn is_id_start(c: char) -> bool { c == '_' || is_xid_start(c) }
fn main(){ let mut n=0; if is_id_start('f') { n+=1; } if is_xid_continue('9') { n+=10; } if is_id_start('1') { n+=100; } if is_id_start('_') { n+=1000; } println!("{}", n); }
