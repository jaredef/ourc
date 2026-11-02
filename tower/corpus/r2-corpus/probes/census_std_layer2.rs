use std::collections::HashMap;

#[derive(Debug)]
enum Tok { Num(i32), Word }

fn classify(s: &str) -> Tok {
    if let Ok(n) = s.parse::<i32>() { Tok::Num(n) } else { Tok::Word }
}

fn main() {
    // labeled break (R1)
    let mut firstpair = 0;
    'scan: for i in 1..6 {
        for j in 1..6 {
            if i * j == 6 { firstpair = i * 10 + j; break 'scan; }
        }
    }
    println!("{}", firstpair);

    // &v[..] full-range slice (R2), bound-local
    let nums = vec![4, 8, 15, 16, 23, 42];
    let whole = &nums[..];
    let total: i32 = whole.iter().sum();
    let head_slice = &nums[..3];
    let head: i32 = head_slice.iter().sum();
    println!("{} {}", total, head);

    // char predicates (R3) + filter/collect
    let text = "a1 b2 c3!";
    let digits: String = text.chars().filter(|c| c.is_numeric()).collect();
    let visible: String = text.chars().filter(|c| !c.is_whitespace()).collect();
    println!("{} {}", digits, visible);

    // f64 rounding (R4)
    let x = 3.7f64;
    let y = -2.5f64;
    println!("{} {} {}", x.floor(), x.ceil(), y.round());

    // std: HashMap<&str> word-count + Result/match + cmp::max + format (numeric BEFORE the caps map)
    let mut freq: HashMap<&str, i32> = HashMap::new();
    for w in "the cat the dog the bird".split_whitespace() {
        *freq.entry(w).or_insert(0) += 1;
    }
    println!("the={}", freq["the"]);

    let words = vec!["10", "hi", "20", "bye"];
    let mut sum = 0;
    for w in &words {
        match classify(w) {
            Tok::Num(n) => sum += n,
            Tok::Word => {}
        }
    }
    let m = std::cmp::max(total, sum);
    println!("{} {:04}", sum, m);

    // capitalize-map (R11 block-map + chars().next().to_ascii_uppercase + &w[1..] + format!) -- FINAL output
    let caps: Vec<String> = words.iter().map(|w| {
        let first = w.chars().next().unwrap().to_ascii_uppercase();
        format!("{}{}", first, &w[1..])
    }).collect();
    println!("{:?}", caps);
}
