fn main() {
    let my_string = String::from("hello world");
    println!("my_string: {}\n", my_string);

    // `first_word`는 `String`의 일부 혹은 전체 슬라이스에 대해 작동합니다
    let word = first_word(&my_string[0..6]);
    println!("first_word(&my_string[0..6]): {}", word);
    let word = first_word(&my_string[..]);
    println!("first_word(&my_string[..]): {}", word);
    // 또한 `first_word`는 `String`의 전체 슬라이스와 동일한 `String`의
    // 참조자에 대해서도 작동합니다
    let word = first_word(&my_string);
    println!("first_word(&my_string): {}\n", word);

    let my_string_literal = "hello world";
    println!("my_string_literal: {}\n", my_string_literal);

    // `first_word`는 문자열 리터럴의 일부 혹은 전체 슬라이스에 대해 작동합니다
    let word = first_word(&my_string_literal[0..6]);
    println!("first_word(&my_string_literal[0..6]): {}", word);
    let word = first_word(&my_string_literal[..]);
    println!("first_word(&my_string_literal[..]): {}", word);

    // 문자열 리터럴은 *곧* 문자열 슬라이스이므로,
    // 아래의 코드도 슬라이스 문법 없이 작동합니다!
    let word = first_word(my_string_literal);
    println!("first_word(my_string_literal): {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}