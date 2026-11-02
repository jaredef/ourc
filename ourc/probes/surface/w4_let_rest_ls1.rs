struct Foo(i64, i64, i64, i64); fn main(){ let Foo(.., d) = Foo(1, 2, 3, 4); let Foo(a, .., e) = Foo(1, 2, 3, 4); let Foo(f, g, ..) = Foo(1, 2, 3, 4); println!("{} {} {} {} {}", d, a, e, f, g); }
