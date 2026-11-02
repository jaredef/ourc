fn main() {
    println!("{:?}", "q\"b\\c\nd\te\rf");
    println!("{:?}", 'x');
    println!("{:?}", '\'');
    println!("{:?}", '\n');
    println!("{:?}", vec!["a\"b", "c"]);
    println!("{}", "plain \" stays");
    println!("{:?}", (1, "a\"b", 'z'));
}
