fn sz<T>(_x: T) -> usize { std::mem::size_of::<T>() } fn main(){ println!("{}", sz(3u16)); }
