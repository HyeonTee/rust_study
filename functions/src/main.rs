fn main() {
    let y = {
        let x = 3;
        x + 1
    };
    
    println!("The value of y is: {y}");
    
    let k = five();
    println!("The value of k is: {k}");
    
    let k = plus_one(five());
    println!("The value of k is: {k}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}