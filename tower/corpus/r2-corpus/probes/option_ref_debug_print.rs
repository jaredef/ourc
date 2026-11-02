// DBG: {:?} on an Option<&T> from find()/first()/last()/min()/max()/position() must print Some(x)/None.
// Those handlers set EOPTREF but not EISOPT, so a bare {:?} saw AOPT=0 and printed the cell pointer.
fn main() {
    let v = vec![3, 1, 2];
    let e: Vec<i32> = vec![];
    println!("{:?} {:?} {:?} {:?} {:?} {:?}",
        v.iter().find(|&&x| x > 1), v.first(), v.iter().max(), v.iter().min(),
        v.iter().position(|&x| x == 2), e.first());
}
