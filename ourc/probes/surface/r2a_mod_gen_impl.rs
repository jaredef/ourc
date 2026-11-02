// R2a P5 guard: generic impls of a type declared in a module -- an inherent `impl<T> G<T>` method and a generic
// `impl<T> Drop for G<T>`. The impl's self type `l$G$T` was cut at its FIRST `$` (-> `l`, not a struct), so the impl kept the
// instance name: `no method get on l$G`, and the Drop never ran (a silent WRONG).
mod l {
    pub struct G<T> {
        pub v: T,
    }
    impl<T> G<T> {
        pub fn get(&self) -> i32 {
            7
        }
    }
    impl<T> Drop for G<T> {
        fn drop(&mut self) {
            println!("dropG");
        }
    }
}
fn main() {
    let g = l::G { v: 3 };
    println!("{}", g.get());
    {
        let _h = l::G { v: String::from("x") };
    }
    println!("end");
}
