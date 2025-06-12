fn main() {
    let some_number = Some(5);
    let some_char = Some('y');
    let some_string = Some("a string");
    let none_number: Option<i32> = None; // 타입 지정 해줘야함
    println!("some number: {:?}", some_number);
    println!("some char: {:?}", some_char);
    println!("some string: {:?}", some_string);
    println!("none number: {:?}\n", none_number);
    
    match divide(10, 0) {
        Some(result) => println!("Result : {}", result),
        None => println!("Cannot divide by zero"),
    }
    
    let x = Some(5);
    let y = 10;
    let sum = x.unwrap() + y;
    println!("Sum : {}", sum);
}

fn divide(x: i32, y: i32) -> Option<i32> {
    if y == 0 {
        None
    } else {
        Some(x / y)
    }
}