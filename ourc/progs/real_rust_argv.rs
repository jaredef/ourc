// Our own driver reads its options from std::env::args, so a compiler meant to compile that driver needs
// argv -- and a main() signature cannot supply it. Every emitted program now takes argc and argv and
// stashes them in globals for the primitives to read; a program that never touches them is unaffected.
fn main(){
    let argv:Vec<String>={ let mut v:Vec<String>=Vec::new(); for a in std::env::args(){ v.push(a); } v };
    let mut n=0; let mut i=1;
    while i<argv.len(){ if argv[i].len()>0 { n=n+1; } i=i+1; }
    println!("{} {}", argv.len()>0, n); }
