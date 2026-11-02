use std::collections::VecDeque;
fn main() {
    let mut q: VecDeque<usize> = VecDeque::new();
    q.push_back(1); q.push_back(2); q.push_back(3);
    let a = q.len();
    let mut order = 0i64;
    while let Some(u) = q.pop_front() {
        order = order * 10 + u as i64;
        if u == 1 { q.push_back(9); }
    }
    println!("{}|{}|{}", order, a, q.is_empty());
}
