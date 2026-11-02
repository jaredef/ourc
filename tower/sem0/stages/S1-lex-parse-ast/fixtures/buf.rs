pub struct Buf {
    data: Vec<u8>,
}

impl Buf {
    pub fn new() -> Buf {
        Buf { data: vec![0; 16] }
    }

    pub fn set(&mut self, i: usize, v: u8) {
        self.data[i] = v;
    }

    pub fn tag(&self) -> char {
        '\n'
    }

    pub fn map_all<T>(items: Vec<T>, f: impl Fn(T) -> u8) -> Vec<u8> {
        let mut out = Vec::new();
        for it in items {
            out.push(f(it));
        }
        out
    }
}
