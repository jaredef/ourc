static FOO: [i64; 4] = [32; 4]; static BAR: [i64; 4] = [32, 32, 32, 32]; const C: [u8; 2] = [1, 2]; fn main() { println!("{} {:?} {} {}", FOO == BAR, FOO, C[1], C.len()); }
