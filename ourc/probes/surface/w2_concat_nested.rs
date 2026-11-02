macro_rules! g { () => { "foo" } } fn main(){ println!("{} {} {}", concat!("a", 1, true, -2), concat!(g!(), concat!("x", "y"), "z"), stringify!(a+b)); }
