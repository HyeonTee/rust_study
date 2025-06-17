use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() {
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];
    // v[99];

    // let greeting_file_result = File::open("hello.txt");
    // let greeting_file = greeting_file_result.unwrap_or_else(|error| match error.kind() {
    //     ErrorKind::NotFound => match File::create("hello.txt") {
    //         Ok(fc) => fc,
    //         Err(e) => panic!("Problem creating the file: {:?}", e),
    //     },
    //     other_error => panic!("Problem opening the file: {:?}", other_error),
    // });

    // let greeting_file_result = File::open("hello.txt")
    //     .expect("hello.txt should be included in this project");

    let username = read_username_from_file().expect("Failed to read username from file");
    println!("Username: {}", username);

    let last_char = last_char_of_first_line("hel\nlo!");
    println!("last_char: {}", last_char.unwrap());
}

fn read_username_from_file() -> Result<String, io::Error> {
    // let username_file_result = File::open("username.txt");
    // let mut username_file = match username_file_result {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    // let mut username = String::new();
    // match username_file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(e) => Err(e),
    // }
    
    // let mut username: String = String::new();
    // File::open("username.txt")?.read_to_string(&mut username)?;
    // Ok(username)
    fs::read_to_string("username.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}