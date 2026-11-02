// LAYER2: str::matches(PAT).count() for a single-char pattern -- occurrences (= split count minus 1),
// reusing __str_count_sep. Was: printed the string unchanged.
fn main() {
    let s = "hello world";
    println!("{}", s.matches("l").count());
    println!("{}", s.matches("o").count());
    println!("{}", s.matches("z").count());
    let dna = "aacgtacgt";
    let a = dna.matches("a").count();
    println!("{}", a + 1);
}
