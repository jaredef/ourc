fn main() {
    let tbl = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let data = b"Man";
    let mut out = String::new();
    let mut i = 0;
    while i < data.len() {
        let b0 = data[i] as u32;
        let b1 = if i + 1 < data.len() { data[i+1] as u32 } else { 0 };
        let b2 = if i + 2 < data.len() { data[i+2] as u32 } else { 0 };
        let n = (b0 << 16) | (b1 << 8) | b2;
        out.push(tbl[((n >> 18) & 63) as usize] as char);
        out.push(tbl[((n >> 12) & 63) as usize] as char);
        out.push(tbl[((n >> 6) & 63) as usize] as char);
        out.push(tbl[(n & 63) as usize] as char);
        i += 3;
    }
    println!("{}", out);
}
