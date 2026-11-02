pub fn keep() {
    use std::str::Chars;
    let _c: Option<Chars> = None;
    #[cfg(feature = "never")]
    {
        use std::ops::Range;
        let _r: Range<u8> = 0..1;
    }
}
