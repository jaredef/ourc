macro_rules! mk_pos {
    ($(pub struct $n:ident ($t:ty);)+) => {
        $( pub struct $n(pub $t); )+
    };
}
mk_pos! {
    pub struct BytePos (u32);
    pub struct CharPos (usize);
}
fn main() {}
