fn main() { let mut st = vec![1, 2, 3]; let mut out = 0; while let Some(x) = st.pop() { out = out * 10 + x; } println!("{}", out); }
