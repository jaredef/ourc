// P-CX-FMTFLAGS / RSPLIT / FMAP-CHAIN / ZIPCHARS / ZIPREV / ZIPFOLD / OPTFIELD-DBG witness (ourc construct-generation,
// 2026-10-25): {:#x}/{:#b}/{:05.2} inside format!/write!; rsplit_once + inline {:?} of split results; filter_map(..)
// followed by map(..) in a collect; for-zip over chars()/rev(); numeric zip(..).fold(..); {:?} of an Option<String> field.
use std::fmt::Write;
#[derive(Debug, Default)] struct O { crate_name: Option<String>, k: u8 }
fn main() {
    let mut out = String::new(); write!(out, "[{:05.2}] [{:#x}] [{:#b}] [{:#o}]", 3.14159, 255, 5, 8).unwrap(); println!("{} {}", out, format!("{:#x}", 4096));
    println!("{:?} {:?} {:?}", "a/b/c.rs".rsplit_once('/'), "x".rsplit_once('/'), "k=v=w".split_once('='));
    if let Some((d, f)) = "a/b/c.rs".rsplit_once('/') { println!("{} | {}", d, f); }
    let s = "  key = value ; other=thing ; bad  ";
    let parts: Vec<(&str, &str)> = s.trim().split(';').filter_map(|p| p.split_once('=')).map(|(a, b)| (a.trim(), b.trim())).collect(); println!("{:?}", parts);
    let n: Vec<i32> = "1,x,3".split(',').filter_map(|t| t.parse::<i32>().ok()).map(|x| x * 10).collect(); println!("{:?}", n);
    let xs = [1, 2, 3];
    for (n, c) in xs.iter().zip("abc".chars()) { print!("{}{} ", c, n); }
    for (n, c) in xs.iter().rev().zip("abc".chars()) { print!("{}{} ", c, n); } println!();
    println!("{}", xs.iter().zip([10, 20, 30].iter()).fold(0, |a, (x, y)| a + x * y));
    let mut o = O::default(); println!("{:?}", o.crate_name); o.crate_name = Some("foo".to_string()); println!("{:?} {}", o.crate_name, o.k);
}
