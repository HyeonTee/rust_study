use std::fmt::Display;

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let s1;
    // let s2;
    {
        let str1: &'static str = "I have a static lifetime.";
        // let str2 = String::from("I have a static lifetime.");
        s1 = str1;
        // s2 = &str2;
    }
    println!("{}", s1);
    // println!("{}", s2);
    // let string1 = String::from("long string is long");
    // 
    // {
    //     let string2 = String::from("xyz");
    //     let result = longest(string1.as_str(), string2.as_str());
    //     println!("The longest string is {}", result);
    // }

    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);
    
    let x = String::from("hello");
    let result;
    {
        let y = String::from("world");
        result = return_x(x.as_str(), y.as_str());
    }
    println!("{}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let i;
    let part;
    {
        // let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        i = ImportantExcerpt { part: first_sentence };
        part = i.announce_and_return_part(&novel);
    }
    // first_sentence는 novel: String 내부 데이터를 가리키는 슬라이스 참조(&str)이므로 first_sentence가 스코프를 벗어나더라도 해당 데이터는 여전히 novel에 있음
    // 그러므로 first_sentence가 사라져도 part는 여전히 접근 가능
    println!("part: {}", part);
    
    let longest = longest_with_an_announcement("긴것", "hello", 12345);
    println!("longest: {}", longest);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // 두 매개변수의 참조자 모두가 유효한 동안에는 반환된 참조자도 유효할 것
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn return_x<'a, T: ?Sized>(x: &'a T, y: &T) -> &'a T {
    // 제네릭 타입 파라미터는 기본적으로 Sized 제약조건을 가짐 -> 컴파일 타임에 정확한 레이아웃을 알기 위함
    // 그러므로 T: ?Sized로 unsized 타입도 받을 수 있도록 수정
    x
}

fn longest_with_an_announcement<'a, T: Display>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}