fn main() { let v = vec![5, 3, 8]; let mut idx: Vec<usize> = (0..v.len()).collect(); idx.sort_by_key(|&i| v[i]); println!("{:?}", idx); }
