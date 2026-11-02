// P-BRD-CLOSURE-BOOL-RET + P-BRD-COLLECT-RESULT-VEC + P-BRD-TDEFAULT-MONO-ROUTE: bool-bodied closures and
// unwrap_or over Option/Result map, collect into Result<Vec<_>,E> with first-Err short-circuit, and inherited trait
// defaults calling overridden/inherited methods for 2 impls (static, tuple-ctor temporary, and dyn).
fn age(n: &str) -> Result<u32, String> { if n.len() == 3 { Ok(n.len() as u32 * 10) } else { Err(format!("bad {}", n)) } }
trait A { fn n(&self) -> String; fn s(&self) -> String { String::from("...") } fn intro(&self) -> String { format!("{}/{}", self.n(), self.s()) } }
struct Dog; struct Fish(u8);
impl A for Dog { fn n(&self) -> String { String::from("dog") } fn s(&self) -> String { String::from("woof") } }
impl A for Fish { fn n(&self) -> String { format!("fish{}", self.0) } }
fn main() {
    let big = |i: u32| i > 3; let ok = |n: &str| age(n).map(|a| a >= 30).unwrap_or(false);
    println!("{} {} {} {}", big(1), big(5), ok("ann"), ok("zz"));
    let all: Result<Vec<u32>, String> = ["ann", "bob"].iter().map(|n| age(n)).collect();
    let bad: Result<Vec<u32>, String> = ["ann", "ze", "bob"].iter().map(|n| age(n)).collect();
    println!("{:?} {:?}", all, bad);
    let zoo: Vec<Box<dyn A>> = vec![Box::new(Dog), Box::new(Fish(2))];
    for a in zoo.iter() { println!("{}", a.intro()); }
    println!("{} {}", Dog.intro(), Fish(7).intro());
}
