struct Point {
    x: i32,
    y: i32,
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

enum Color {
    Rgb(u8, u8, u8),
    Hsv(u8, u8, u8),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

enum HelloMessage {
    Hello { id: u8 },
}

fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
    
    stack.push(100);
    
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
    
    let point = (3, 5);
    print_coordinates(&point);
    
    // Matching Literals
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    
    // Matching Named Variables
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {y}", x);
    
    // Multiple Patterns
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    
    // Matching Ranges of Values with ..=
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
    
    // let msg = Message::Write("hello".to_owned());
    // match msg {
    //     Message::Quit => {
    //         println!("The Quit variant has no data!");
    //     }
    //     Message::Move { x, y } => {
    //         println!("Move in the x direction {} and in the y direction {}", x, y);
    //     }
    //     Message::Write(text) => println!("Text message: {}", text),
    //     Message::ChangeColor(r, g, b) => {
    //         println!("Change the color to red {}, green {}, and blue {}", r, g, b);
    //     }
    // }
    
    let msg = Message::ChangeColor(Color::Hsv(0, 126, 35));
    match msg { 
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change the color to hue {}, saturation {}, and value {}", h, s, v)
        }
        _ => (),
    }

    let mut setting_value = Some(1);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string: {:?}", s);
    }

    println!("{:?}", s);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
    
    // Match Guard
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);

    let x = 6;
    let y = false;

    // 4 | 5 | (6 if y) 와 같음
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
    
    let msg = HelloMessage::Hello { id: 4 };
    match msg {
        HelloMessage::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        HelloMessage::Hello { id: 10..=12, .. } => {
            println!("Found an id in another range")
        },
        HelloMessage::Hello { id } => println!("Found some other id: {}", id),
    }
}