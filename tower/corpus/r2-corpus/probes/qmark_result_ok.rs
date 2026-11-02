// Conservative-strip witness for CHECK N4: legitimate `?` uses must STILL compile+run
// (Result? in a Result fn, Option? in an Option fn, parse()? ). rustc accepts all of these.
fn g(r: Result<i32, String>) -> Result<i32, String> { let x = r?; Ok(x + 1) }
fn h(o: Option<i32>) -> Option<i32> { let y = o?; Some(y * 2) }
fn p(s: &str) -> Result<i32, std::num::ParseIntError> { let n: i32 = s.parse()?; Ok(n) }
fn main() {
    println!("{}|{:?}|{}", g(Ok(4)).unwrap(), h(Some(5)), p("7").unwrap());
}
