// LAYER2 P1 (user-enum slice): String/&str variant payload restored at the match/if-let binder
// (was: bound as bare i32 -> printed the descriptor pointer). VFSTR marks the field; SK=3 on the slot.
enum Msg { Text(String), Pair(String, i32), Quit }
fn describe(m: Msg) -> String {
    match m {
        Msg::Text(s) => format!("text {}", s),
        Msg::Pair(s, n) => format!("pair {} {}", s, n),
        Msg::Quit => String::from("quit"),
    }
}
fn main() {
    println!("{}", describe(Msg::Text(String::from("hi"))));
    println!("{}", describe(Msg::Pair(format!("k{}", 2), 9)));
    println!("{}", describe(Msg::Quit));
    let e = Msg::Text(format!("v{}", 7));
    if let Msg::Text(s) = e {
        println!("got {}", s);
    }
}
