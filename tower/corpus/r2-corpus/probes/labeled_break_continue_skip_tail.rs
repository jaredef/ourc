// LAYER2 P4: a labeled break/continue set by a NESTED loop must skip the REST of the outer loop body
// (the statements after the inner loop), not run them. Was: the outer-body tail (a println after the
// inner for) ran even when `break 'outer` had already fired.
fn main() {
    'outer: for i in 0..3 {
        for j in 0..3 {
            if i + j == 2 { break 'outer; }
        }
        println!("tail {}", i);   // must NOT print: break 'outer fires at i=0,j=2
    }
    let mut hits = 0;
    'o2: for i in 0..3 {
        for j in 0..3 {
            if j == 2 { continue 'o2; }
            hits += 1;
        }
        println!("unreachable {}", i);   // continue 'o2 skips this
    }
    println!("hits {}", hits);
}
