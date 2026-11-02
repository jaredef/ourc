fn p(a: &str) -> String { match a { "-n" => "n".to_string(), s if s.starts_with("--e=") => s[4..].to_string(), o => format!("bad {}", o) } }
fn main() { println!("{} {} {}", p("-n"), p("--e=ab"), p("zz")); }
