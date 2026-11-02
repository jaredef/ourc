enum Shape{ Circle{r:i32}, Rect{w:i32,h:i32} }
fn area(s:&Shape)->i32{ match s { Shape::Circle{r}=>r*r*3, Shape::Rect{w,h}=>w*h } }
fn main(){ println!("{}",area(&Shape::Rect{w:3,h:4})+area(&Shape::Circle{r:2})); }
