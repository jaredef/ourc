struct Triple<T> { x: T, y: T, z: T }
fn box_it<T>(x: Triple<T>) -> Box<Triple<T>> { return Box::new(x); }
fn pick<T, U>(_a: Triple<T>, b: Triple<U>) -> Triple<U> { b }
fn main() {
    let x = box_it(Triple { x: 1, y: 2, z: 3 });
    assert_eq!(x.y, 2);
    let s = box_it(Triple { x: String::from("a"), y: String::from("bc"), z: String::from("d") });
    assert_eq!(s.y, "bc");
    println!("{} {} {}", x.x + x.z, s.y, s.y.len());
    let p = pick(Triple { x: 1, y: 2, z: 3 }, Triple { x: String::from("p"), y: String::from("q"), z: String::from("r") });
    println!("{}", p.y);
}
