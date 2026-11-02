// R2a P6 ORPHAN guard: a LOCAL GENERIC trait for a std type is allowed -- `Foo<i32>` spells `Foo$i32`, whose last
// `$` segment is the type argument; the orphan check read `i32` as the trait and refused it (E0117 false positive)

trait Foo<T> {
    fn get(&self) -> T;
}

impl Foo<i32> for i32 {
    fn get(&self) -> i32 { *self }
}

fn main() {
    let x: i32 = 1;
    println!("{}", Foo::<i32>::get(&x));
}
