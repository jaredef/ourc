fn main() {
    let mut fact = 1;
    for i in 1..=10 { fact = fact * i; }
    println!("10! = {}", fact);
    let mut primes = 0;
    for n in 2..50 {
        let mut is_p = 1;
        for d in 2..n { if n % d == 0 { is_p = 0; } }
        if is_p == 1 { primes = primes + 1; }
    }
    println!("primes under 50 = {}", primes);
}
