type ThinVec<T> = Vec<T>;
type P<T> = Box<T>;
struct Arm { binds: ThinVec<u32> }
enum E { Match(P<i64>, ThinVec<Arm>) }
fn go(e: &E) -> i32 {
    match e {
        E::Match(_s, arms) => {
            let mut s = 0;
            for arm in arms.iter() {
                for b in arm.binds.iter() { s = s + *b as i32; }
            }
            s
        }
    }
}
fn main() {
    let mut arms: ThinVec<Arm> = Vec::new();
    let mut b: ThinVec<u32> = Vec::new(); b.push(2); b.push(3); arms.push(Arm { binds: b });
    let e = E::Match(Box::new(9), arms);
    println!("{}", go(&e));
}
