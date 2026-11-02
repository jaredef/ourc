// The tag-dispatch chain used to emit an arm for EVERY lifted closure, including ones whose declared
// parameter count cannot match the call site: a two-parameter closure got a one-argument call from each
// of the prelude's one-argument callbacks. Those arms are dead -- the tag only matches when the value
// really is that closure -- but they are ill-formed IR. Measured on this program: 6 of 25 closure calls
// were arity-mismatched before, 0 of 19 after, with identical output.
fn main(){
    let base=10;
    let add2=|a:i32, b:i32| a+b+base;
    let v=vec![1,2,3,4];
    let doubled:Vec<i32>=v.iter().map(|x| x*2).collect();
    let pos=v.iter().position(|x| *x==3);
    let any=v.iter().any(|x| *x>3);
    println!("{} {} {:?} {}", add2(1,2), doubled[3], pos, any); }
