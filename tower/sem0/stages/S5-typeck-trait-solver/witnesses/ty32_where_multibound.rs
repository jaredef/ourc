trait A{} trait B{}
fn f<T>(x: T) where T: A + B {}
