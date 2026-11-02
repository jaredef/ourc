fn main() {
    let n = 5;
    let inf = 1_000_000i64;
    // adjacency matrix
    let g = vec![
        vec![0, 4, 1, inf, inf],
        vec![inf, 0, inf, 1, inf],
        vec![inf, 2, 0, 5, inf],
        vec![inf, inf, inf, 0, 3],
        vec![inf, inf, inf, inf, 0],
    ];
    let mut dist = vec![inf; n];
    let mut done = vec![false; n];
    dist[0] = 0;
    for _ in 0..n {
        let mut u = usize::MAX;
        let mut best = inf;
        for v in 0..n {
            if !done[v] && dist[v] < best { best = dist[v]; u = v; }
        }
        if u == usize::MAX { break; }
        done[u] = true;
        for v in 0..n {
            if g[u][v] < inf && dist[u] + g[u][v] < dist[v] {
                dist[v] = dist[u] + g[u][v];
            }
        }
    }
    println!("{}", dist[4]);
}
