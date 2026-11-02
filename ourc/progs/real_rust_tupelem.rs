// A tuple's element type lives in the tuple's NAME, which a tag cannot carry -- so when the checker
// cannot name an element it must answer UNKNOWN, not int. Binding one as int collided with a String
// field, a bool field and an as_str call.
struct A { variant:String, wild:bool }
fn pick(k:i32)->(String,bool){ if k==0 { (String::from("x"), true) } else { (String::from("_"), false) } }
fn main(){
    let (vn, wild)=pick(0);
    let a=A{ variant:vn, wild:wild };
    let (vn2, w2)=pick(1);
    let s=String::from(vn2.as_str());
    let t=pick(0);
    println!("{} {} {} {} {} {}", a.variant, a.wild, s, w2, t.0, t.1); }
