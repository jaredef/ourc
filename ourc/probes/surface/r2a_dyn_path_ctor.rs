// R2a P5 guard: a trait object named through a module path (`dyn l::Sh`), a tuple-struct constructor used inside its
// own module (`Sq(3)` in `mod l`), and a type closing two generics right before `=` with no space (`>>=`).
mod l {
    pub trait Sh {
        fn area(&self) -> i32;
    }
    pub struct Sq(pub i32);
    impl Sh for Sq {
        fn area(&self) -> i32 {
            self.0 * self.0
        }
    }
    pub fn mk() -> Sq {
        Sq(4)
    }
}
fn main() {
    let v: Vec<Box<dyn l::Sh>>=vec![Box::new(l::Sq(3))];
    let w:Vec<Vec<i32>>=vec![vec![l::mk().0]];
    println!("{} {}", v[0].area(), w[0][0]);
}
