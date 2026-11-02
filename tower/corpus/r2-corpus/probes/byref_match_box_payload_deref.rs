// D004 IC-3: a by-ref match/if-let payload that is itself a pointer needs skip-one-then-load on `**e`
// (the 1st `*` is the ergonomic ref-skip, the 2nd a real Box/ref load). Was: SFORREF no-op'd BOTH.
fn main() {
    let o: Option<Box<i32>> = Some(Box::new(9));
    let x = 7;
    let r: Option<&i32> = Some(&x);
    let a = match &o { Some(e) => **e, None => -1 };       // Option<Box<T>> by-ref
    let b = if let Some(e) = &o { **e } else { 0 };        // if-let by-ref, same
    let c = match &r { Some(e) => **e, None => -1 };       // Option<&T> by-ref, same skip-one-then-load
    println!("{} {} {}", a, b, c);
}
