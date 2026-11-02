fn parse(v: Vec<i32>, o: Option<i32>) -> Result<i32, ()> {
    let mut total = 0;
    for x in v { total = total + x; }
    let base = o?;
    while let Some(n) = o { total = total + n; }
    if let Some(k) = o { total = total + k; } else { total = total - 1; }
    let Some(g) = o else { return Ok(0); };
    let r = match total { 0 | 1 => 10, 2..=9 => 20, m @ 100 => m, _ => total * 2 - base };
    let f = |y| y + base;
    Ok(r + f(g))
}
fn main() {}
