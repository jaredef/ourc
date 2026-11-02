// R2a P5 TBASE guard: drop GLUE of module types -- a struct with a Drop field and an enum with a Drop payload, both declared
// in a module. gbase cut `l$W` / `l$E` at the first `$`, so the glue found no definition and dropped nothing (silent WRONGs).
mod l {
    pub struct D(pub &'static str);
    impl Drop for D {
        fn drop(&mut self) {
            println!("drop {}", self.0);
        }
    }
    pub struct W {
        pub d: D,
        pub x: i32,
    }
    pub enum E {
        A(D),
        N,
    }
}
fn main() {
    let _w = l::W { d: l::D("w"), x: 1 };
    {
        let _e = l::E::A(l::D("e"));
    }
    println!("end");
}
