fn g() -> Option<String> { Some(String::from("w")) }
fn main() { let o: Option<&str> = Some("q"); let p = Some("r"); let s: Option<String> = Some(String::from("t"));
  let r: Result<i64, String> = Err(String::from("bad")); let f: Option<f64> = Some(2.5);
  println!("{} {} {} {} {} {:?} {:?} {} {}", o.unwrap_or("z"), o.unwrap(), p.unwrap(), s.clone().unwrap(), g().unwrap(), s, r, r.is_err(), f.unwrap()); }
