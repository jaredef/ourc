macro_rules! gen_fn {
    ($fn_name:ident, $t:ty) => {
        pub fn $fn_name(x: $t) -> $t { x }
    };
}
gen_fn!(alpha, u8);
gen_fn!(beta, u16);
