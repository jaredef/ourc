trait Tr { fn m(&self) -> u8; }
fn f<T>(x: T) -> u8 where T: Tr { x.m() }
