enum T { T1(()), T2(()) }
enum V { V1(isize), V2(bool), V3(char) }
fn foo(x: (T, V)) -> String {
    match x {
        (T::T1(()), V::V1(i)) => format!("V1({})", i),
        (T::T2(()), V::V2(b)) => format!("V2({})", b),
        _ => String::new(),
    }
}
fn main() {
    let v = V::V2(true);
    let V::V2(b) = v else { panic!() };
    println!("{} {} {}", b, foo((T::T1(()), V::V1(99))), foo((T::T2(()), V::V2(true))));
}
