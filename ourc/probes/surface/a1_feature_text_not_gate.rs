// #![feature(never_type)] in a comment
#![allow(unused)]
#[derive(Clone, Copy)] struct P(i64);
fn main(){ let s = "#![feature(x)]"; let p = P(3); println!("{} {}", s, p.0); }
