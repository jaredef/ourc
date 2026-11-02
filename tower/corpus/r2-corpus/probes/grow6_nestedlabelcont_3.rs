fn main() {
    let mut sum = 0;
    'outer: for i in 0..5 {
        for j in 0..5 {
            if j == 2 { continue 'outer; }
            sum += i * 10 + j;
        }
    }
    println!("{}", sum);
}
