fn roman(mut n: i64) -> String {
    let vals = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
    let syms = ["M","CM","D","CD","C","XC","L","XL","X","IX","V","IV","I"];
    let mut out = String::new();
    let mut i = 0;
    while i < vals.len() {
        while n >= vals[i] {
            out.push_str(syms[i]);
            n -= vals[i];
        }
        i += 1;
    }
    out
}
fn main() {
    println!("{}", roman(1994));
}
