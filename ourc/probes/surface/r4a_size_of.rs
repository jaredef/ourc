use std::marker::PhantomData;
use std::mem;
#[repr(C)]
struct P { a: u8, b: u32, c: u16 }
struct Q { a: u8, b: u32, c: u16 }
struct R<'a> { s: &'a str, n: u8 }
struct W { v: Vec<u8>, s: String, q: Q }
fn main() {
    println!("{} {} {} {}", mem::size_of::<P>(), mem::size_of::<Q>(), mem::size_of::<Option<&u8>>(), mem::size_of::<(u8, u64)>());
    println!("{} {} {} {} {}", mem::size_of::<&str>(), mem::size_of::<Box<u8>>(), mem::size_of::<[u16; 5]>(), mem::size_of::<Option<u32>>(), mem::size_of::<Option<Box<u8>>>());
    println!("{} {} {} {} {}", mem::size_of::<R>(), mem::size_of::<W>(), mem::size_of::<()>(), mem::size_of::<PhantomData<u64>>(), mem::align_of::<P>());
    println!("{} {} {} {} {}", mem::size_of::<u128>(), mem::size_of::<&[u8]>(), mem::size_of::<Option<bool>>(), mem::size_of::<(u8, (u16, u8))>(), mem::size_of::<[(u8, u32); 3]>());
    let v: Vec<Vec<u8>> = Vec::<Vec<u8>>::new();
    println!("{}", v.len());
}
