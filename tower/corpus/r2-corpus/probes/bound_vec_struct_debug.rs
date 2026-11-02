// R25b: {:?} on a BOUND Vec<Struct> local must print [P { .. }, ..] (was element pointers). The Vec-Debug
// Vec<Struct> branch (AVELT) only saw the element struct-type for an INLINE vec!; a bound-var dispatch now
// threads VELT = SVELT so the print arg captures AVELT correctly (mirrors R25's DVELENUM).
#[derive(Debug)]
struct P { x: i32, y: i32 }
fn main() {
    let v = vec![P { x: 1, y: 2 }, P { x: 3, y: 4 }];
    println!("{:?}", v);
}
