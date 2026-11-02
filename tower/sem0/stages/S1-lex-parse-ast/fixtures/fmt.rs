pub struct Printer {
    indent: usize,
}

impl Printer {
    pub fn raw() -> &'static str {
        r"C:\path"
    }

    pub fn magic() -> u8 {
        b'A'
    }

    pub fn parse_id(s: &str) -> u32 {
        let n = s.parse::<u32>().unwrap();
        n
    }

    pub fn make() -> Vec<u8> {
        Vec::<u8>::new()
    }

    pub fn check(x: u32) -> bool {
        matches!(x, 1 | 2 | 3)
    }
}
