// cx_brd_patconst: const + assoc-const patterns (P-PAT-LITERAL, rung361) -- an assoc-const PATH pattern is a
// value test (not an enum variant), a `const S: &str` pattern is a literal test (not a binding).
const NAME: &str = "rustc";
const MAGIC: u32 = 0xCAFE;
struct Limits;
impl Limits { const MAX: i32 = 100; }
fn kw(s: &str) -> u8 { match s { "fn" => 1, NAME => 3, "" => 0, _ => 9 } }
fn main() {
    println!("{} {} {} {}", kw("fn"), kw("rustc"), kw(""), kw("x"));
    for n in [-1i32, 100, 5] { println!("{}", match n { -1 => "m1", Limits::MAX => "max", _ => "mid" }); }
    for v in [0u32, MAGIC, 7] { println!("{}", match v { 0 => "zero", MAGIC => "magic", _ => "other" }); }
}
