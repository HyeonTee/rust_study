use std::env;
use std::process;

use minigrep::Config;

// 하나의 함수가 하나의 작업에 대한 책임을 지도록
// 설정 변수들을 하나의 구조체로 묶어서 목적을 분명히
// 에러 메시지를 구체적으로, 모든 에러 처리 코드를 한 곳에
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    println!("Searching for {} in file {}...\n", config.query, config.file_path);
    
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}