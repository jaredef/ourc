enum State { Idle, Running, Done }
fn step(s: State, ev: i64) -> State {
    match s {
        State::Idle => if ev == 1 { State::Running } else { State::Idle },
        State::Running => if ev == 2 { State::Done } else { State::Running },
        State::Done => State::Done,
    }
}
fn code(s: &State) -> i64 {
    match s { State::Idle => 0, State::Running => 1, State::Done => 2 }
}
fn main() {
    let events = [0, 1, 0, 2, 1];
    let mut s = State::Idle;
    let mut trace = 0i64;
    for e in events.iter() {
        s = step(s, *e);
        trace = trace * 10 + code(&s);
    }
    println!("{}", trace);
}
