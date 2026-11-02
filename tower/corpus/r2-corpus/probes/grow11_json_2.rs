enum J { Num(i64), Arr(Vec<J>) }
fn pretty(v: &J, out: &mut String) {
    match v {
        J::Num(n) => out.push_str(&n.to_string()),
        J::Arr(items) => {
            out.push('[');
            let mut first = true;
            for it in items {
                if !first { out.push(','); }
                first = false;
                pretty(it, out);
            }
            out.push(']');
        }
    }
}
fn main() {
    let v = J::Arr(vec![J::Num(1), J::Arr(vec![J::Num(2), J::Num(3)]), J::Num(4)]);
    let mut s = String::new();
    pretty(&v, &mut s);
    println!("{}", s);
}
