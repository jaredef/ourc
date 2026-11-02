fn main() {
    let input = [1i64, 0, 1, 1, 0, 1];
    let mut state = 0i64;
    let mut count = 0i64;
    let mut i = 0;
    while i < input.len() {
        if state == 0 {
            if input[i] == 1 { state = 1; } 
        } else {
            if input[i] == 0 { state = 0; count += 1; }
        }
        i += 1;
    }
    println!("{} {}", state, count);
}
