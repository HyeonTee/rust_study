fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
    
    let mut s2 = "bye";
    println!("{}", s2);
    s2 = "world";
    println!("{}", s2);
}
