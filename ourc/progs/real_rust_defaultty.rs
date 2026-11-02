// `T::default()` names its own type; String is normalized to our spelling so one rule sees one name
fn main(){ let s = String::default(); let b = bool::default(); let x = f64::default();
    let mut v: Vec<i64> = Vec::default(); v.push(2);
    println!("[{}] {} {} {} {}", s, b, x, v.len(), v[0]); }
