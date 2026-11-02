pub struct Parser {
    pos: usize,
}

impl Parser {
    #[inline]
    pub fn peek(&self) -> u8 {
        0
    }

    pub fn parse_num(s: &str) -> Result<u32, Error> {
        let n = s.parse()?;
        Ok(n)
    }

    pub fn make_adder(base: u32) -> impl Fn(u32) -> u32 {
        move |x| x + base
    }

    pub fn classify(&self, c: char) -> u32 {
        if c == '\t' || c == ' ' {
            1
        } else {
            2
        }
    }
}
