fn main() {
    let mut r = 1;
    while r <= 5 {
        let mut c = 0;
        while c < r { print!("{}", '*'); c = c + 1; }
        println!("");
        r = r + 1;
    }
}
