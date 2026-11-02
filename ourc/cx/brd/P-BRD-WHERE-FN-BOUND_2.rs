fn ap<F>(f: F, xs: &[String]) -> Vec<usize> where F: for<'a> Fn(&'a str) -> &'a str { xs.iter().map(|s| f(s.as_str()).len()).collect() }
fn main() { let xs = vec!["hello".to_string(), "ab".to_string()]; println!("{:?}", ap(|s| s.trim_start_matches('h'), &xs)); }
