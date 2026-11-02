fn main() { let mut g = vec![vec![0; 3]; 2]; g[1][2] = 5; g[0][0] += 1; let s: i64 = g.iter().map(|r| r.iter().sum::<i64>()).sum(); println!("{:?} {}", g, s); }
