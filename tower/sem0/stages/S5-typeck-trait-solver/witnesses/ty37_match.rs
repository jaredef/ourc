fn lit(x: u8) -> u8 { match x { 0 => 1, _ => 2 } }
fn bind(x: u8) -> u8 { match x { n => n } }
fn guard(x: u8) -> u8 { match x { n if true => 1, _ => 2 } }
fn wide(x: u8) -> u16 { match x { 0 => 1, _ => 2 } }
fn main() {}
