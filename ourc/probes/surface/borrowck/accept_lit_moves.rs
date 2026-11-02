struct T { x: String }
struct P { x: i32 }
fn main() {
    let s = String::from("a");
    let t = T { x: s };
    let a = 3;
    let p = P { x: a };
    let c = String::from("c");
    let o = Some(c.clone());
    println!("{} {} {} {} {:?}", t.x, a, p.x, c, o);
}
