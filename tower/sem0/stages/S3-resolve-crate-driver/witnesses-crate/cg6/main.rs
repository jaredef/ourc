extern crate deputil;
mod geometry;
use deputil::base;
use deputil::Sym;
use geometry::Pt;
use geometry::area;
fn main() {
    let p = Pt { x: 3, y: 4 };
    println!("{}", p.norm());          // 25
    println!("{}", area(6, 7));         // 42
    let s = Sym { id: 5 };
    println!("{}", s.code());           // 15
    println!("{}", base(20));           // 1020
}
