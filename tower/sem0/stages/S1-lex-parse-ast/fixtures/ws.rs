pub fn is_whitespace(c: char) -> bool {
    match c {
        'a' | 'b' | 'c' => true,
        _ => false,
    }
}
pub fn is_id_start(c: char) -> bool {
    c == 'x' || c == 'y'
}
pub fn class(c: char) -> u32 {
    if is_whitespace(c) {
        1
    } else if is_id_start(c) {
        2
    } else {
        0
    }
}
