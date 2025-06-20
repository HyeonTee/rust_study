fn main() {
    let s1 = String::from("hello");
    takes_ownership(s1);

    let x = 5;
    makes_copy(x);

    println!("outer x: {}", x);

    let s2 = String::from("hello");
    let s3 = gives_ownership();
    let s4 = takes_and_gives_back(s3);

    println!("{}, {}", s2, s4);

    let mut vec = vec![10, 20, 30, 40, 50];
    let e = vec.get(1).unwrap_or(&&vec[0]);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("inner x: {}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("world!");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}