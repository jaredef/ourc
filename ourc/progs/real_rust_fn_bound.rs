// A type parameter bounded by a CALLABLE is our `Fn` type -- in the generics list, in a where clause,
// and under a higher-ranked `for<'a>` quantifier. Once the parameter type is `Fn` the type parameter has
// no work left, so it leaves the generic list: the function is not generic in our model at all.
fn apply<F: Fn(u8) -> u8>(f: F) -> u8 { f(6) }
fn apply2<F>(f: F) -> u8 where F: Fn(u8) -> u8 { f(7) }
fn apply3<F>(f: F) -> u8 where F: for<'a> Fn(&'a u8) -> u8 { f(&8) }
fn twice<F: Fn(u8) -> u8>(f: F, x: u8) -> u8 { f(f(x)) }
fn main() {
    println!("{} {} {}", apply(|x: u8| x), apply2(|x: u8| x + 1), apply3(|x: &u8| *x));
    println!("{}", twice(|x: u8| x * 2, 3));
}
