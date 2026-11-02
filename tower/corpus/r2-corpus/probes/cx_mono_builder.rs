// P-CX-MONO witness (ourc construct-generation, 2026-10-24): a generic fn instantiated at >1 type routes each call to
// its instance (total$Sq / total$Rect); `-> &mut Self` builder chains; Stack<String>::top() -> Option<T> at the call
// site resolves T; `dyn Trait<Assoc = X>` names the trait.
trait Shape { type Unit; fn area(&self) -> i64; fn name(&self) -> String { "shape".to_string() } }
struct Sq(i64); struct Rect { w: i64, h: i64 }
impl Shape for Sq { type Unit = u8; fn area(&self) -> i64 { self.0 * self.0 } fn name(&self) -> String { format!("sq{}", self.0) } }
impl Shape for Rect { type Unit = u16; fn area(&self) -> i64 { self.w * self.h } }
fn total<S: Shape>(xs: &[S]) -> i64 { xs.iter().map(|s| s.area()).sum() }
struct Stack<T> { items: Vec<T> }
impl<T: Clone + std::fmt::Debug> Stack<T> { fn new() -> Self { Stack { items: Vec::new() } } fn push(&mut self, t: T) -> &mut Self { self.items.push(t); self } fn top(&self) -> Option<T> { self.items.last().cloned() } }
fn main() {
    println!("{} {}", total(&[Sq(2), Sq(3)]), total(&[Rect { w: 2, h: 5 }]));
    let mut s: Stack<String> = Stack::new(); s.push("a".into()).push("b".into()).push("c".into()); println!("{:?} {}", s.top(), s.items.len());
    let n: Stack<u32> = Stack { items: vec![1, 2] }; println!("{:?}", n.top());
    let d: Vec<Box<dyn Shape<Unit = u8>>> = vec![Box::new(Sq(5)), Box::new(Sq(1))]; println!("dyn {}", d.iter().map(|x| x.area()).sum::<i64>());
}
