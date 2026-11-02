enum Shape{ Rect{w:i64,h:i64}, Circle{r:i64} }
fn area(s:&Shape)->i64{ match s { Shape::Rect{w,h}=>w*h, Shape::Circle{r}=>r*r*3 } }
fn main(){ let a=Shape::Rect{w:3,h:4}; let b=Shape::Circle{r:5}; println!("{}", area(&a)+area(&b)); }
