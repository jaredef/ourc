pub enum Node {
    Leaf(u32),
    Pair { left: u32, right: u32 },
    Empty,
}

impl Node {
    pub fn sum(&self) -> u32 {
        match self {
            Node::Leaf(v) => *v,
            Node::Pair { left, right } => left + right,
            Node::Empty => 0,
        }
    }

    pub fn split(p: (u32, u32)) -> u32 {
        let (a, b) = p;
        a + b
    }

    pub fn drain(it: &mut Iter) -> u32 {
        let mut total = 0;
        while let Some(x) = it.next() {
            total += x;
        }
        total
    }
}
