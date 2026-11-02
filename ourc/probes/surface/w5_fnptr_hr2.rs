fn pick<'a>(a: &'a str, b: &'a str) -> &'a str { if a.len() > b.len() { a } else { b } } fn main(){ let f: for<'x> fn(&'x str, &'x str) -> &'x str = pick; println!("{}", f("ab", "c")); }
