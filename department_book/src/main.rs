use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};

type DepartmentBook = HashMap<String, Vec<String>>;

fn main() {
    let mut book = load_data("department_book.json");

    println!("부서-직원 관리 시스템 (종료하려면 'exit' 입력)");

    loop {
        println!("명령을 입력하세요 (예: Add Sales Alice, List Sales, ListAll):");

        let mut input = String::new();
        io::stdin().lock().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0].to_lowercase().as_str() {
            "add" if parts.len() == 3 => {
                let department = parts[1].to_string();
                let name = parts[2].to_string();
                book.entry(department.clone())
                    .or_default()
                    .push(name.clone());
                println!("'{}' 부서에 '{}' 추가됨", department, name);
            }
            "list" if parts.len() == 2 => {
                let department = parts[1];
                match book.get(department) {
                    Some(employees) => {
                        println!("'{}' 부서 직원:", department);
                        for name in employees {
                            println!("- {}", name);
                        }
                    }
                    None => println!("'{}' 부서를 찾을 수 없습니다", department),
                }
            }
            "listall" if parts.len() == 1 => {
                for (dept, employees) in &book {
                    println!("{} 부서:", dept);
                    for name in employees {
                        println!("- {}", name);
                    }
                }
            }
            _ => {
                println!("잘못된 명령입니다. 형식: Add/List/ListAll");
            }
        }
    }

    save_data("department_book.json", &book);
}

fn load_data(path: &str) -> DepartmentBook {
    if Path::new(path).exists() {
        let file = File::open(path).expect("파일 열기 실패");
        serde_json::from_reader(file).unwrap_or_default()
    } else {
        HashMap::new()
    }
}

fn save_data(path: &str, data: &DepartmentBook) {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .expect("파일 열기 실패");
    serde_json::to_writer_pretty(file, data).expect("파일 저장 실패");
}
