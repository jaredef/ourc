fn main() { let mut st: Vec<(String, i64)> = vec![(String::from("a"), 1), (String::from("bc"), 2)]; while let Some((s, n)) = st.pop() { println!("{} {} {}", s, n, s.len()); } }
