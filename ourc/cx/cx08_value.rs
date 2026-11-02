// cx08 value model: rustc-shaped value plumbing -- &mut scalar through closures (m8), tuple-of-Vec
// returns from loops (LOOPTUPRET), Option<Box<T>> linked lists, mem::{swap,replace,take}, slices, enums.
use std::mem;
#[derive(Debug, Default, Clone)] struct Node { v: i64, next: Option<Box<Node>> }
fn partition(v: &[i64]) -> (Vec<i64>, Vec<i64>) { let mut a = Vec::new(); let mut b = Vec::new(); for &x in v { if x % 2 == 0 { a.push(x) } else { b.push(x) } } (a, b) }
fn bump(c: &mut i64, by: i64) -> i64 { *c += by; *c }
fn list(n: i64) -> Option<Box<Node>> { let mut h = None; for i in (1..=n).rev() { h = Some(Box::new(Node { v: i, next: h })); } h }
fn sum_list(mut l: &Option<Box<Node>>) -> i64 { let mut s = 0; while let Some(n) = l { s += n.v; l = &n.next; } s }
fn reverse(mut l: Option<Box<Node>>) -> Option<Box<Node>> { let mut prev = None; while let Some(mut n) = l { l = n.next.take(); n.next = prev; prev = Some(n); } prev }
#[derive(Debug, Clone, PartialEq)] enum Val { I(i64), S(String), L(Vec<Val>), Unit }
fn deep(v: &Val) -> i64 { match v { Val::I(i) => *i, Val::S(s) => s.len() as i64, Val::L(xs) => xs.iter().map(deep).sum(), Val::Unit => 0 } }
fn main() {
    let (ev, od) = partition(&[1, 2, 3, 4, 5, 6, 7]); println!("{:?} {:?}", ev, od);
    let mut counter = 10; { let mut inc = |k: i64| { counter += k; counter }; inc(1); inc(5); } println!("closure-mut {}", counter);
    let mut c = 0; for i in 0..4 { bump(&mut c, i); } let r = &mut c; *r *= 3; println!("bump {}", c);
    let mut total = 0i64; let mut add = |x: &i64| total += *x; [4, 5, 6].iter().for_each(&mut add); println!("for_each-mut {}", total);
    let l = list(5); println!("list sum {} rev-head {:?}", sum_list(&l), reverse(l).map(|n| n.v));
    let mut a = vec![1, 2]; let mut b = vec![9]; mem::swap(&mut a, &mut b); let old = mem::replace(&mut a, vec![7, 7, 7]); let taken = mem::take(&mut b);
    println!("swap {:?} {:?} {:?} {:?}", a, b, old, taken);
    let arr = [5, 1, 4, 2, 3]; let s = &arr[1..4]; let (l2, r2) = arr.split_at(2); println!("slice {:?} {} {:?} {:?} {:?}", s, s.len(), l2, r2, arr.iter().rev().skip(1).step_by(2).collect::<Vec<_>>());
    let v = Val::L(vec![Val::I(3), Val::S("abcd".into()), Val::L(vec![Val::I(10), Val::Unit])]); println!("deep {} eq {}", deep(&v), v == v.clone());
    let mut grid = vec![vec![0i64; 3]; 2]; for (i, row) in grid.iter_mut().enumerate() { for (j, x) in row.iter_mut().enumerate() { *x = (i * 3 + j) as i64; } } println!("grid {:?}", grid);
    let mut opt: Option<i64> = Some(3); if let Some(x) = opt.as_mut() { *x += 39; } let shown = format!("{:?}", opt); println!("opt {} {:?}", shown, opt.take().map(|x| x * 2).unwrap_or(-1));
    let words = vec!["a".to_string(), "bb".to_string()]; let lens: Vec<(usize, &str)> = words.iter().map(|w| (w.len(), w.as_str())).collect(); println!("{:?}", lens);
}
