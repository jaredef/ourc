// `!` is the logical not on a bool and the BITWISE not on an integer. With one scalar width, the integer
// form is faithful exactly where the high bits cannot escape -- the mask idiom -- so it is admitted under
// a bitwise operator and refused elsewhere.
fn align_to(x:i64, a:i64)->i64{ let m=a-1; (x+m) & (!m) }
fn align2(x:i64, a:i64)->i64{ (x+a-1) & (!(a-1)) }
fn main(){
    let m:i64=7; let x:i64=21;
    let b=true;
    println!("{} {} {} {} {} {} {}",
        align_to(5,8), align_to(16,8), align2(1,4), align2(17,16), x | (!m), x ^ (!m), !b); }
