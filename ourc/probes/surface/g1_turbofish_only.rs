fn sz<T>() -> usize { std::mem::size_of::<T>() } fn main(){ println!("{}", sz::<u64>()); }
