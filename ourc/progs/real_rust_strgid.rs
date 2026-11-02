// A string constant's global name is derived from its content, so the derivation must never name two
// literals the same. The old step was *33, which for a two-character literal reduces to c1*33+c2 and
// collides when c1 differs by one and c2 by 33 -- ".." and "-O" are exactly that pair. And the name was
// the hash's DECIMAL, printed by a loop that runs while the value is positive, so with one scalar width
// every hash with the high bit set printed as nothing at all.
fn main(){
    let a=".."; let b="-O"; let c="&&"; let d="$h";
    let long="a literal long enough that its djb2 hash sets the high bit of a 64-bit accumulator, which is what used to make its global name empty";
    let long2="another literal long enough that its djb2 hash sets the high bit of a 64-bit accumulator, so the two used to share one name";
    println!("{} {} {} {} {} {} {} {}", a, b, c, d, a.len(), long.len(), long2.len(), long==long2); }
