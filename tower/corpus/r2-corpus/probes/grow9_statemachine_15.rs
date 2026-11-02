#[derive(PartialEq)]
enum State { Idle, Running, Done }
fn step(s: State) -> State {
    match s {
        State::Idle => State::Running,
        State::Running => State::Done,
        State::Done => State::Done,
    }
}
fn main() {
    let mut s = State::Idle;
    let mut n = 0;
    while s != State::Done { s = step(s); n += 1; }
    println!("{}", n);
}
