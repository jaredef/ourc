// C1/C11: a `loop` is a VALUE -- a result slot is bound around it and `break v` assigns the slot at the
// point the parser already knows which loop it is in (no AST rewrite, so none of the boxed-move shapes
// semc miscompiles). `Vec::pop` answers an Option. And the dummy label after a break branch carries a
// UNIQUE id: two breaks at the same op index in different blocks used to emit the same label, which LLVM
// merged into one block and reported as a terminator in the middle of it.
fn main() {
    let mut i = 0;
    let v = loop { i += 1; if i == 4 { break i * 10; } };
    println!("{}", v);
    let mut w = vec![1, 2, 3];
    let mut n = 0;
    while let Some(x) = w.pop() { if x == 2 { break; } n += x; }
    println!("{}", n);
    println!("{:?} {:?}", w.pop(), Vec::<i32>::new().pop());
    let mut k = 0;
    let z = loop { k += 1; if k > 2 { break k; } };
    println!("{}", z);
}
