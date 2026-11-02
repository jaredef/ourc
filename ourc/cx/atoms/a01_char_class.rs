fn main() { for c in ['a', 'Z', '_', '1', ' ', 'ö', 'λ', '"'] { println!("{} {} {} {} {:?}", c.is_alphabetic(), c.is_alphanumeric(), c.is_ascii_digit(), c.is_whitespace(), c.to_digit(10)); } }
