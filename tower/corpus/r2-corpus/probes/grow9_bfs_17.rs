use std::collections::VecDeque;
fn main() {
    let adj: Vec<Vec<usize>> = vec![vec![1,2], vec![3], vec![3], vec![]];
    let mut visited = vec![false; adj.len()];
    let mut q: VecDeque<usize> = VecDeque::new();
    q.push_back(0);
    visited[0] = true;
    let mut order = 0;
    while let Some(u) = q.pop_front() {
        order = order * 10 + u as i64;
        for &v in &adj[u] {
            if !visited[v] { visited[v] = true; q.push_back(v); }
        }
    }
    println!("{}", order);
}
