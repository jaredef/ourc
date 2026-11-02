pub struct Literal;
pub struct Diagnostic;
macro_rules! suffixed_int_literals {
    ($($name:ident => $kind:ident,)*) => ($(
        #[stable(feature = "x", since = "1.0.0")]
        pub fn $name(n: $kind) -> Literal { Literal }
    )*)
}
macro_rules! diagnostic_method {
    ($name:ident, $level:expr) => {
        #[unstable(feature = "y", issue = "1")]
        pub fn $name<T: Into<String>>(self, message: T) -> Diagnostic { Diagnostic }
    };
}
impl Literal {
    fn new() -> Self { Literal }
    suffixed_int_literals! {
        u8_suffixed => u8,
        u16_suffixed => u16,
        i8_suffixed => i8,
    }
    diagnostic_method!(error, Level::Error);
    diagnostic_method!(warning, Level::Warning);
}
fn main() {}
