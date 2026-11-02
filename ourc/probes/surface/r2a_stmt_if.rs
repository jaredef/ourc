// R2a P4 guard: an `if` in STATEMENT position has no value -- branches of different types (incl. an else-if chain and a
// Vec::remove statement) are fine; a VALUE-position if still needs agreeing arms (not probed here: that is a reject).
fn main() {
    let mut ns: Vec<String> = vec![String::from("a"), String::from("b")];
    let mut fv: Vec<i64> = Vec::new();
    let c = true;
    if c { ns.remove(0); } else { fv.push(1); }
    if !c { 5; } else { "x"; }
    let x = 3;
    if x > 5 { fv.push(9); } else if x > 2 { ns.push(String::from("z")); } else { fv.push(0); }
    let s = if x > 2 { "big" } else { "small" };
    println!("{} {} {}", ns.len(), fv.len(), s);
}
