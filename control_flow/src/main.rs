fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
    
    let mut number = 10;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
        if number == 0 {
            println!("LIFTOFF!!!");
            number -= 1;
        }
        
        if number == -10 {
            println!("what?");
            break;
        }
    }
    
    let arr = [10, 20, 30, 40, 50];
    for element in arr {
        println!("the value is: {}", element);
    }
    
    for i in (1..4).rev() {
        println!("{}!", i);
    }
    println!("LIFTOFF!!!");
}