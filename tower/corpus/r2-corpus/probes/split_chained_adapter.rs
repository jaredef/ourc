// R22c: split(SEP).<map/filter/sum/max/...> must materialize the &str Vec and route the chained adapter
// through the Vec pipeline (was unsupported -> returned the split cursor -> garbage). collect/count unchanged.
fn main() {
    let s = "12,34,56";
    let sum: i32 = s.split(",").map(|x| x.parse::<i32>().unwrap()).sum();
    let t = "a,bb,ccc";
    let c = t.split(",").filter(|w| w.len() > 1).count();
    let longest = "hello world foo".split(" ").map(|w| w.len()).max().unwrap();
    println!("{} {} {}", sum, c, longest);
}
