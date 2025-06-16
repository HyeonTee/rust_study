use std::collections::HashMap;
use rand::Rng;
use std::io;

fn main() {
    println!("=============== < Vector > ===============");
    let mut v = vec![1, 2, 3];

    let _v1 = v[0];

    let v2 = &mut v[1];
    *v2 = 10;
    println!("{:?}", v[1]);


    let vec = vec![100, 32, 9233, 600];
    for i in &vec {
        println!("{}", i);
    }

    let mut vec2 = vec![100, 32, 9233, 600];
    for i in &mut vec2 {
        *i += 50;
    }
    println!("vec2: {:?}", vec2);

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for cell in &row {
        match cell {
            SpreadsheetCell::Int(v) => println!("int: {}", v),
            SpreadsheetCell::Float(v) => println!("float: {}", v),
            SpreadsheetCell::Text(v) => println!("text: {}", v),
        }
    }

    println!("=============== < String > ===============");
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push(' ');

    s1.push_str(s2); // push_str이 소유권을 가져가지 않음
    println!("s1: {}", s1);
    println!("s2: {}", s2);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1은 소유권이 이동되었음
    println!("s3: {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("s: {}", s);

    let hello = "Здравствуйте";
    let s = &hello[0..6]; // 각 문자가 2바이트, 0..3으로 슬라이스하면 panic
    println!("s: {}", s);

    for b in "Зд".bytes() {
        println!("{}", b);
    }

    for c in "Здра".chars() {
        println!("{}", c);
    }

    println!("=============== < HashMap > ===============");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    scores.entry(String::from("Green")).or_insert(30);
    scores.entry(String::from("Yellow")).or_insert(70);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.insert(String::from("Blue"), 25);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("score: {}", score);

    let field_name = String::from("Favorite color");
    // drop(field_name);
    let field_value = String::from("Red");
    let mut map = HashMap::new();
    map.insert(&field_name, field_value);
    println!("map: {:?}", map);
    println!("field_name: {}", field_name);
    // println!("field_value: {}", field_value);

    let text = "hello world wonderful world";
    let mut text_map = HashMap::new();
    for word in text.split_whitespace() {
        let count = text_map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("textMap: {:?}", text_map);

    println!("=============== < Exercise > ===============");
    let mut rng = rand::rng();

    let mut random_numbers: Vec<i32> = (0..16)
        .map(|_| rng.random_range(1..=10))
        .collect();

    random_numbers.sort();
    println!("random_numbers: {:?}", random_numbers);

    let len = random_numbers.len();
    let median = if len % 2 == 1 {
        random_numbers[len / 2] as f64
    } else {
        let mid = len / 2;
        (random_numbers[mid - 1] + random_numbers[mid]) as f64 / 2.0
    };
    println!("median: {}", median);
    
    let mut count_map = HashMap::new();
    for number in random_numbers {
        let count = count_map.entry(number).or_insert(0);
        *count += 1;
    }
    let mut count = 0;
    let mut mode = 0;
    for (key, value) in count_map {
        if count < value {
            count = value;
            mode = key;
        }
    }
    
    println!("mode: {}\n", mode);

    let mut input = String::new();
    loop {
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        input = input.trim().to_string();
        pig_latinize(&mut input);
        println!("{}", input);
        input.clear();
        break
    }
}

fn pig_latinize(s: &mut String) {
    if s.is_empty() {
        return;
    }

    let first_char = s.chars().next().unwrap();
    let rest = s.chars().skip(1).collect::<String>();

    if is_vowel(first_char) {
        *s = format!("{}-hay\n", s);
    } else {
        *s = format!("{}-{}ay\n", rest, first_char);
    }
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' |
                'A' | 'E' | 'I' | 'O' | 'U')
}
