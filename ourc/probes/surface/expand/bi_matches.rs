fn main() { let o = Some(3); println!("{} {}", matches!(o, Some(x) if x > 2), matches!(o, None)); }
