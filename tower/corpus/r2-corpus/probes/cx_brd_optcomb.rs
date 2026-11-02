// P-BRD-OPTCOMB-TY: Option combinator result typing (map(closure) / filter / their unwrap) and a std-type path fn
// passed as a map argument (`.map(str::to_uppercase)` = UFCS method form).
fn lookup(id: u32) -> Option<&'static str> { match id { 1 => Some("ann"), 2 => Some("bob"), 3 => Some(""), _ => None } }
fn main() {
    println!("{:?} {:?}", lookup(1).map(|s| s.to_uppercase()), lookup(3).filter(|s| !s.is_empty()));
    println!("{} {}", lookup(2).filter(|s| s.len() > 1).unwrap(), lookup(1).map(str::to_uppercase).unwrap());
    let v: Vec<String> = (0..4).map(|i| lookup(i).filter(|s| !s.is_empty()).map(str::to_uppercase).unwrap_or_else(|| "?".into())).collect();
    println!("{:?}", v);
}
