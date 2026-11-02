// char-iterator .collect() into String (generic path) -- regression guard for
// TI3e-i: `.collect()` defaulted to a Vec; a String target (via `let s: String`
// annotation or `.collect::<String>()` turbofish) must pack the char-Vec into a
// {buf,len} String descriptor, not return the Vec (printed as a pointer).
fn main() {
    let a: String = ('a'..='e').collect();
    let b: String = vec!['x', 'y', 'z'].into_iter().collect();
    let c = ('0'..='3').collect::<String>();
    println!("{} {} {}", a, b, c);
    // Vec collect must still work (no regression)
    let v: Vec<i32> = (1..4).map(|x| x * 2).collect();
    println!("{}", v.iter().sum::<i32>());
}
