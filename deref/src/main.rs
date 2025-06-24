use std::ops::Deref;

// T 타입의 요소 하나를 가진 튜플 구조체
#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    // let x = 5;
    // let y = MyBox::new(x);
    // 
    // assert_eq!(5, x);
    // assert_eq!(5, *y);
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
