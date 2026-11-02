fn main() { let mut st: Vec<(i64, i64)> = vec![(1, 2), (3, 4)]; let mut acc = 0; while let Some((a, b)) = st.pop() { acc = acc * 10 + a + b; } println!("{}", acc); }
