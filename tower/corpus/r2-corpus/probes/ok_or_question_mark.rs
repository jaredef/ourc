// LAYER4 leaf-crate frontier: `.ok_or(err)?` / `.ok_or_else(|| err)?` -- Option -> Result then `?`.
// rustc_lexer's unescape scan_escape uses `chars.next().ok_or(EscapeError::LoneSlash)?`. ok_or(_else) builds a
// Result cell (Some->Ok / None->Err) but did not mark EOPTRES, so a chained `?` in a `-> Result` fn was rejected
// as "`?` on Option". Now ok_or/ok_or_else set EOPTRES=1 so the `?` is accepted.
fn first(x: Option<char>) -> Result<i32, i32> {
    let c = x.ok_or(9)?;
    Ok(c as i32)
}
fn firstl(x: Option<i32>) -> Result<i32, i32> {
    let c = x.ok_or_else(|| 7)?;
    Ok(c + 1)
}
fn main() {
    match first(Some('A')) { Ok(n) => println!("ok {}", n), Err(e) => println!("err {}", e) }
    match first(None) { Ok(n) => println!("ok {}", n), Err(e) => println!("err {}", e) }
    match firstl(Some(3)) { Ok(n) => println!("ok {}", n), Err(e) => println!("err {}", e) }
    match firstl(None) { Ok(n) => println!("ok {}", n), Err(e) => println!("err {}", e) }
}
