macro_rules! mk_alias {
    (Self = $t:ident, Prim = $p:ty) => {
        pub type $t = $p;
    };
}
mk_alias! { Self = MyU8, Prim = u8 }
mk_alias! { Self = MyU16, Prim = u16 }
fn main() {}
