struct S { a: i32, b: i32 }
fn shorthand(a: i32, b: i32) -> S { S { a, b } }
fn explicit(a: i32, b: i32) -> S { S { a: a, b: b } }
fn mixed(a: i32) -> S { S { a, b: 5 } }
fn withbase(s: S) -> S { S { a: 1, ..s } }

fn main() {}
