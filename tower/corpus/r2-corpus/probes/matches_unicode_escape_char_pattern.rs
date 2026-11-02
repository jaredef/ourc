fn is_ws(c: char) -> bool { matches!(c, '\u{0009}' | '\u{0020}' | '\u{0085}' | '\u{200E}') }
fn main() { println!("{} {} {}", is_ws('f'), is_ws(' '), is_ws('\t')); }
