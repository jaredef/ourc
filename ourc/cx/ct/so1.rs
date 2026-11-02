#[derive(Debug, Clone, PartialEq)] enum T { Named(String, Vec<T>), Param(usize) }
struct Impl { trait_: String, nparams: usize, where_: Vec<(String, usize)> }
fn main() { let im = Impl { trait_: "Clone".into(), nparams: 1, where_: vec![("Clone".into(), 0)] }; let tr = "Clone"; println!("{} {}", im.trait_ != tr, im.trait_ == tr);
  let mut s: Vec<Option<T>> = vec![None; im.nparams]; s[0] = Some(T::Param(4)); println!("{:?}", s);
  let ok = im.where_.iter().all(|(wt, p)| match &s[*p] { Some(t) => { println!("{} {:?}", wt, t); true } None => false }); println!("{}", ok); }
