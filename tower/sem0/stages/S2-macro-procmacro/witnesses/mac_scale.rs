macro_rules! sq { ($x:expr) => { $x * $x } }
macro_rules! pick { () => { 0 }; (one) => { 1 }; ($x:expr) => { $x } }
macro_rules! sum { ($($x:expr),*) => { { let mut t = 0; $( t = t + $x; )* t } } }
macro_rules! cast { ($v:expr, $t:ty) => { $v as $t } }
macro_rules! first { ($p:pat, $e:expr) => { match $e { $p => 1, _ => 0 } } }
macro_rules! run { ($b:block) => { $b } }
macro_rules! lit { ($l:literal) => { $l } }
macro_rules! rec { (0) => { 0 }; ($n:expr) => { 1 + rec!(0) } }
macro_rules! nest { ($x:expr) => { sq!($x) + 1 } }
macro_rules! both { ($a:expr, $b:expr) => { sq!($a) + sq!($b) } }
fn a() -> i32 { sq!(3) }
fn b() -> i32 { pick!() + pick!(one) + pick!(7) }
fn c() -> i32 { sum!(1, 2, 3, 4) }
fn d() -> i64 { cast!(5, i64) }
fn e() -> i32 { first!(1, 2) }
fn f() -> i32 { run!({ 8 }) }
fn g() -> i32 { lit!(42) }
fn h() -> i32 { rec!(5) }
fn i() -> i32 { nest!(4) }
fn j() -> i32 { both!(2, 3) }
fn k() -> bool { matches!(9, 1 | 9) }
fn l() -> i32 { sum!(sq!(2), sq!(3)) }
fn main() {}
