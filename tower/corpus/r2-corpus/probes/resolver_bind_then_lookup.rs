type ThinVec<T> = Vec<T>;
struct Rib { names: ThinVec<String>, ids: ThinVec<u32> }
struct R { ribs: ThinVec<Rib>, next_local: u32 }
impl R {
    fn bind(&mut self, name: &str) -> u32 {
        let id = self.next_local; self.next_local += 1;
        let top = self.ribs.len() - 1;
        self.ribs[top].names.push(String::from(name));
        self.ribs[top].ids.push(id);
        id
    }
    fn look(&self, name: &str) -> bool {
        let mut i = 0;
        while i < self.ribs[0].names.len() {
            if self.ribs[0].names[i].as_str() == name { return true; }
            i += 1;
        }
        false
    }
}
fn main() {
    let mut r = R { ribs: vec![Rib{names: Vec::new(), ids: Vec::new()}], next_local: 0 };
    r.bind("a");
    println!("{} {}", r.look("a"), r.look("z"));
}
