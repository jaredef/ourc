use std::rc::Rc;
fn twice(r: Rc<i32>) -> i32 { *r * 2 }
fn main() {
    let a = Rc::new(7);
    let b = Rc::clone(&a);
    println!("{} {} {}", a, b, Rc::strong_count(&a));
    {
        let c = a.clone();
        println!("{}", Rc::strong_count(&c));
    }
    println!("{} {}", Rc::strong_count(&a), *a + 1);
    let x = Rc::new(1);
    let y = Rc::new(1);
    let z = x.clone();
    println!("{} {} {}", Rc::strong_count(&x), Rc::strong_count(&y), z);
    println!("{} {}", twice(a.clone()), Rc::strong_count(&a));
    let m = a;
    println!("{} {}", m, Rc::strong_count(&m));
    let s = Rc::new("hi");
    let t = s.clone();
    println!("{} {} {}", s, t.len(), Rc::strong_count(&s));
}
