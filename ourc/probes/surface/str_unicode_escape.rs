fn main() { let a = "\u{2603}"; let b = "\u{2a10}\u{2A01}\u{2Aa0}"; let c = "\\{20}"; let d = "x\u{41}y"; println!("[{}] {} [{}] {} [{}] {} [{}]", a, a.len(), b, b.len(), c, c.len(), d); }
