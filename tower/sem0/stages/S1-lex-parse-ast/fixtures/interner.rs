pub struct Interner {
    names: Vec<String>,
}

impl Interner {
    pub fn intern(&mut self, s: &str) -> u32 {
        let n = self.names.len() as u32;
        self.names.push(s.to_string());
        n
    }

    pub fn lookup(&self, i: u32) -> &str {
        &self.names[i as usize]
    }

    pub fn build(items: &[u32]) -> Interner {
        let mut out = Interner { names: Vec::new() };
        for &x in items {
            let d = x * 2;
            out.names.push(format!("n{}", d));
        }
        out
    }

    pub fn total(&self) -> usize {
        self.names.iter().map(|s| s.len()).sum()
    }
}
