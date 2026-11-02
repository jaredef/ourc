// C-SIZEOF: a fact about the TARGET, so the numbers are rustc's own, stated once in a width table
fn main(){ println!("{} {} {} {} {}", std::mem::size_of::<u8>(), std::mem::size_of::<i32>(),
    std::mem::size_of::<u64>(), std::mem::size_of::<bool>(), std::mem::size_of::<char>()); }
