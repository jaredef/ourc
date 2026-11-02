fn get(i: i32) -> Result<Option<i32>, String> {
    if i < 0 { Err("neg".to_string()) }
    else if i == 0 { Ok(None) }
    else { Ok(Some(i*2)) }
}
fn main() {
    let mut out = 0;
    for i in [-1,0,3] {
        match get(i) {
            Ok(Some(v)) => out += v,
            Ok(None) => out += 100,
            Err(_) => out += 1000,
        }
    }
    println!("{}", out);
}
