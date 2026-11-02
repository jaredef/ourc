// DBG: a BARE bound tuple local `let t=(..); println!("{:?}", t)` must print (a, b), not the cell pointer.
// A bound tuple local recorded STUPTY but the bare-identifier read set ETUPTY without EISTUP, so the print
// arg took the scalar path. Now a bare (non-.field) tuple local restores EISTUP/ETUPN.
fn main() {
    let t = (1, 2);
    let u = (10, 20, 30);
    let s = t.0 + t.1;            // field access still works
    println!("{:?} {:?} {}", t, u, s);
}
