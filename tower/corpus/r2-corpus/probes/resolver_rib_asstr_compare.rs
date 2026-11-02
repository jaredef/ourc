struct Rib { names: Vec<String> }
struct R { ribs: Vec<Rib> }
impl R {
    fn look(&self, name: &str) -> bool { let mut k = self.ribs.len(); while k > 0 { k -= 1; let rib = &self.ribs[k]; let mut i = 0; while i < rib.names.len() { if rib.names[i].as_str() == name { return true; } i += 1; } } false }
}
fn main() { let mut ribs: Vec<Rib> = Vec::new(); ribs.push(Rib{names: vec![String::from("a")]}); let r = R{ribs: ribs}; println!("{} {}", r.look("a"), r.look("z")); }
