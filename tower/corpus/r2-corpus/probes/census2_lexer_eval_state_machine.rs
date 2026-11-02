use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
enum Token { Num(i32), Plus, Minus, Ident(String) }

fn lex(s: &str) -> Vec<Token> {
    let mut out = Vec::new();
    for w in s.split_whitespace() {
        if w == "+" { out.push(Token::Plus); }
        else if w == "-" { out.push(Token::Minus); }
        else if let Ok(n) = w.parse::<i32>() { out.push(Token::Num(n)); }
        else { out.push(Token::Ident(String::from(w))); }
    }
    out
}

fn eval(tokens: &[Token]) -> i32 {
    let mut acc = 0;
    let mut op = 1;
    for t in tokens {
        match t {
            Token::Num(n) => acc += op * n,
            Token::Plus => op = 1,
            Token::Minus => op = -1,
            Token::Ident(_) => {}
        }
    }
    acc
}

trait Summary {
    fn count(&self) -> usize;
    fn label(&self) -> String;
    fn report(&self) -> String { format!("{}: {}", self.label(), self.count()) }
}

struct Bag { items: Vec<i32> }
impl Summary for Bag {
    fn count(&self) -> usize { self.items.len() }
    fn label(&self) -> String { String::from("bag") }
}

fn main() {
    let toks = lex("3 + 4 - 1 + foo 2");
    println!("{:?}", toks);
    println!("{}", eval(&toks));

    let mut freq: HashMap<&str, i32> = HashMap::new();
    for w in "a b a c a b".split_whitespace() {
        *freq.entry(w).or_insert(0) += 1;
    }
    println!("{}", freq["a"]);

    let b = Bag { items: vec![10, 20, 30] };
    println!("{}", b.report());

    let words = vec!["apple", "banana", "cherry"];
    let caps: Vec<String> = words.iter().map(|w| {
        let first = w.chars().next().unwrap().to_ascii_uppercase();
        format!("{}{}", first, &w[1..])
    }).collect();
    println!("{:?}", caps);

    let nums: Vec<i32> = (1..=5).collect();
    let evens_sum: i32 = nums.iter().filter(|&&x| x % 2 == 0).sum();
    let product: i32 = nums.iter().product();
    println!("{} {}", evens_sum, product);
}
