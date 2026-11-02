macro_rules! s {
    () => { 0 };
    ($h:expr $(, $t:expr)*) => { $h + s!($($t),*) };
}
macro_rules! p {
    ($h:expr $(, $t:expr)*) => { $h $(* $t)* };
}
macro_rules! v {
    ($($x:expr),* $(,)?) => { vec![$($x),*] };
}
fn main() {
    println!("{} {} {:?}", s!(1, 2, 3, 4), p!(2, 3, 4), v!(5, 6,));
}
