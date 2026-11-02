macro_rules! mk_enum {
    ($($v:ident,)*) => {
        pub enum LangItem { $($v,)* }
    };
}
mk_enum! { Sized, Copy, Clone, }
fn main() {}
