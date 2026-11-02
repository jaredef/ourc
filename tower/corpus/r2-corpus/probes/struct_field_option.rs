// P-STRUCT-FIELD-OPTION: a struct/tuple-struct field of type Option<T> dispatches its Option methods
// (.take() with correct leave-None, .is_some()/.is_none()/.unwrap_or()). rustc newtype `S(Option<T>)`
// (OnDrop) + named Option fields are pervasive in the rustc front end (rustc_data_structures OnDrop::disable).
struct OnDrop(Option<i32>);
struct Slot { cur: Option<i32>, n: i32 }
fn main() {
    let mut d = OnDrop(Some(9));
    let taken = d.0.take();                 // returns Some(9), leaves None
    let mut s = Slot { cur: Some(4), n: 7 };
    let c = s.cur.take();                    // named-field take
    println!("{} {} {} {} {}",
        taken.unwrap_or(0), d.0.is_none(),   // 9 true
        c.unwrap_or(0), s.cur.is_none(),     // 4 true
        s.n);                                // 7
}
