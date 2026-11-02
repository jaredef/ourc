// C-STRGUARD: a str comparison whose operand is not a pointer is always a defect -- an int handle reached
// a str parameter -- and it used to segfault inside strcmp. The guarded helper says so and exits 101
// instead, the same discipline the IR self-check and the index check already follow. This fixture is the
// CONTROL: every legitimate str comparison must still answer exactly, and none may trip the guard.
fn eq(a:&str, b:&str)->bool{ a==b }
fn main(){
    let a="xy"; let b=String::from("xy"); let c=String::from("");
    let v=vec![String::from("p"), String::from("q")];
    println!("{} {} {} {} {} {}",
        a==b.as_str(), a=="zz", eq(&b, "xy"), eq(&c, ""), v[0].as_str()=="p", v[1]==String::from("z")); }
