enum St { Idle, Run(i32), Done(i32) }
fn step(s: St) -> i32 { match s { St::Idle => 1, St::Run(n) => n * 2, St::Done(n) => n + 100 } }
fn main() { std::process::exit(step(St::Run(5)) + step(St::Done(3)) + step(St::Idle)); }