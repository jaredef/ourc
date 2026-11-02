// as_str, to_string and to_owned are the identity in VALUE but their TYPE is always str, whatever the
// receiver. Answering the receiver's type instead meant a String built from an unnameable receiver came
// out unnameable itself -- and String::from is erased at parse time, so nothing downstream restored the
// name. A push_str on the result then reached the emitter as an undefined call.
fn pick()->(String, i32){ (String::from("tr"), 7) }
fn main(){
    let (trn, k)=pick();
    let mut dn=String::from(trn.as_str()); dn.push_str("$"); dn.push_str("cn");
    let mut up=trn.to_string(); up.push_str("!");
    println!("{} {} {} {}", dn, dn.len(), up, k); }
