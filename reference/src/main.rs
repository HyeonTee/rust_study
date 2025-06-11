fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("1. 불변 참조자 사용");
    println!("The length of '{}' is {}.\n", s1, len);

    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("2. 가변 참조자 사용");
    println!("{}\n", s2);

    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("3. 불변 참조자 여러번 사용");
    println!("r1: {}, r2: {}\n", r1, r2);
    
    let r3 = &mut s;
    println!("3. 불변 참조자 여러번 사용");
    println!("r3: {}", r3);
    r3.push_str(", r3");
    println!("{}", r3);
    println!("s: {}\n", s);
    
    println!("어떤 값에 대한 불변 참조자가 있는 동안 가변 참조자 만드는 것 불가능");
    println!("여러 개의 불변 참조자를 만드는 것은 가능\n");
    
    let reference_to_nothing = no_dangle();
    println!("{}", reference_to_nothing);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn no_dangle() -> String { // &String을 반환해버리면, 참조자를 반환하고 s는 스코프를 벗어나버림. dangling pointer 생김. 컴파일에러로 막음
    let s = String::from("hello");
    
    s
}