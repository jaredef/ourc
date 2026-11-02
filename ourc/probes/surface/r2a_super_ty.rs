// R2a P6 guard: `super::T` / `self::T` in a TYPE resolve relative to the current module, even when another module declares a
// struct of the same name (they collapsed to the first-registered `T`).
mod m { pub struct T { pub a: i32 } }
mod n {
    pub struct T { pub y: i32 }
    pub mod sub {
        pub struct T { pub z: i32 }
        pub fn own(t: &self::T) -> i32 { t.z }
        pub fn up(t: &super::T) -> i32 { t.y }
        pub mod deep { pub fn up2(t: &super::super::T) -> i32 { t.y * 2 } }
    }
}
fn main() {
    let t = n::T { y: 9 };
    let u = m::T { a: 1 };
    println!("{} {} {} {}", n::sub::own(&n::sub::T { z: 4 }), n::sub::up(&t), n::sub::deep::up2(&t), u.a);
}
