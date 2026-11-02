struct Foo { x: isize, y: isize }
fn main(){ let g = |Foo { x: x, y: _y }: Foo| x + 1; let h = |Foo { y, .. }: Foo| y * 2; let r = g(Foo { x: 1, y: 2 }) + h(Foo { x: 5, y: 7 }); println!("{}", r); }
