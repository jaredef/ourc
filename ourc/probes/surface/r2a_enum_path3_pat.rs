// R2a P4 guard: module-qualified enum paths in match patterns (3 and 4 segments, payload, nested, matches!).
mod a {
    pub enum T { X(i32), Y }
    pub mod b { pub enum U { P, Q } }
}
fn f(t: a::T) -> i32 { match t { a::T::X(v) => v, a::T::Y => 2 } }
fn g(u: a::b::U) -> i32 { match u { a::b::U::P => 1, a::b::U::Q => 2 } }
fn h(t: Option<a::T>) -> i32 { match t { Some(a::T::X(v)) => v * 10, Some(a::T::Y) => 3, None => 0 } }
fn main() {
    println!("{} {} {} {} {} {} {} {}", f(a::T::X(7)), f(a::T::Y), g(a::b::U::P), g(a::b::U::Q),
        h(Some(a::T::X(4))), h(Some(a::T::Y)), h(None), matches!(a::T::Y, a::T::Y));
}
