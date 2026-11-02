fn arr<T>(x: T) -> [T; 1] { [x] }
fn g() { let a: [u8; 1] = arr(1); }
