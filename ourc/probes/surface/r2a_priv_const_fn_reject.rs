// R2a P5 guard: a PRIVATE const fn of a module stays private (E0603)
mod l { const fn sq(x: usize) -> usize { x * x } } fn main(){ println!("{}", l::sq(3)); }
