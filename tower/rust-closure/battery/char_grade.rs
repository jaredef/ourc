fn grade(score: i32) -> char {
    if score >= 90 { 'A' } else if score >= 80 { 'B' } else if score >= 70 { 'C' } else { 'F' }
}
fn main() {
    let scores = [95, 82, 71, 50];
    let mut i = 0;
    while i < 4 { println!("score {} -> {}", scores[i as usize], grade(scores[i as usize])); i = i + 1; }
}
