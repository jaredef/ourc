type ThinVec<T> = Vec<T>;
#[derive(Clone, Copy)]
enum Ns { Value, Type }
#[derive(Clone, Copy)]
enum Res { Local(u32), Def(u32), PrimTy(u8), Err }
struct Binding { name: String, ns: Ns, res: Res }
struct Resolver { scope: ThinVec<Binding>, next_local: u32, diags: ThinVec<String> }
impl Resolver {
    fn bind_val(&mut self, name: &str) -> u32 {
        let id = self.next_local; self.next_local += 1;
        self.scope.push(Binding { name: String::from(name), ns: Ns::Value, res: Res::Local(id) });
        id
    }
    fn bind_def(&mut self, name: &str, ns: Ns, defid: u32) {
        self.scope.push(Binding { name: String::from(name), ns: ns, res: Res::Def(defid) });
    }
    // resolve in a specific namespace: search scope top-down for a matching (name, ns)
    fn resolve_ns(&mut self, name: &str, ns: Ns) -> Res {
        let mut i = self.scope.len();
        while i > 0 {
            i -= 1;
            let b = &self.scope[i as usize];
            let same_ns = match (b.ns, ns) { (Ns::Value, Ns::Value) => true, (Ns::Type, Ns::Type) => true, _ => false };
            if same_ns && b.name.as_str() == name { return b.res; }
        }
        if name == "i32" { return Res::PrimTy(1); }
        // E0425: unresolved -> record a diagnostic + Err (recovery)
        let nsn = match ns { Ns::Value => "value", Ns::Type => "type" };
        self.diags.push(format!("E0425: cannot find {} `{}`", nsn, name));
        Res::Err
    }
}
fn dump_res(r: &Res) -> String {
    match r { Res::Local(n) => format!("Local(_{})", n), Res::Def(n) => format!("Def({})", n), Res::PrimTy(p) => format!("PrimTy({})", p), Res::Err => String::from("Err") }
}
fn main() {
    let mut r = Resolver { scope: Vec::new(), next_local: 0, diags: Vec::new() };
    // struct S : a TYPE `S` and a VALUE ctor `S` coexist in distinct namespaces
    r.bind_def("S", Ns::Type, 0);
    r.bind_def("S", Ns::Value, 0);
    r.bind_def("foo", Ns::Value, 1);
    let _x = r.bind_val("x");
    // a resolution map: (name, ns) -> Res
    println!("S(type) -> {}", dump_res(&r.resolve_ns("S", Ns::Type)));
    println!("S(value) -> {}", dump_res(&r.resolve_ns("S", Ns::Value)));
    println!("x(value) -> {}", dump_res(&r.resolve_ns("x", Ns::Value)));
    println!("foo(value) -> {}", dump_res(&r.resolve_ns("foo", Ns::Value)));
    println!("i32(type) -> {}", dump_res(&r.resolve_ns("i32", Ns::Type)));
    println!("x(type) -> {}", dump_res(&r.resolve_ns("x", Ns::Type)));   // x is a value, not a type -> Err
    println!("nope(value) -> {}", dump_res(&r.resolve_ns("nope", Ns::Value)));
    // dump the collected diagnostics (recovery: multiple)
    for d in r.diags.iter() { println!("{}", d); }
}
