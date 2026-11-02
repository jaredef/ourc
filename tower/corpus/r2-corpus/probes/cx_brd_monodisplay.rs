// P-BRD-BLANKET-MONO + P-BRD-MONO-DISPLAY + P-BRD-FMT-FLOAT: blanket impls instantiated per receiver type, {} of a
// generic-param value inside a mono instance (&T, blanket self, a float field), and a plain {} float in format!.
use std::fmt::Display;
trait Loud { fn loud(&self) -> String; }
impl<T: Display> Loud for T { fn loud(&self) -> String { format!("{}!", self) } }
trait Area { fn area(&self) -> u32; }
trait Report { fn report(&self) -> String; }
impl<T: Area> Report for T { fn report(&self) -> String { format!("area {}", self.area()) } }
struct Sq(u32); impl Area for Sq { fn area(&self) -> u32 { self.0 * self.0 } }
fn show_ref<T: Display + ?Sized>(x: &T) -> String { format!("[{}]", x) }
struct Wrap<T> { inner: T, n: i32 }
impl<T: Display> Wrap<T> { fn show(&self) -> String { format!("<{}:{}>", self.n, self.inner) } }
fn main() {
    println!("blanket display: {} {} {}", "hi".loud(), 42.loud(), 'x'.loud());
    println!("blanket user trait: {}", Sq(4).report());
    println!("&T display: {} {} {}", show_ref("s"), show_ref(&7), show_ref(&'c'));
    let w = Wrap { inner: 2.5f64, n: 1 };
    println!("float field: {}", w.show());
    let x = 0.125f64;
    println!("format float: {}", format!("<{}> {}", x, 3.75));
}
