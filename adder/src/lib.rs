pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        }

        Guess { value }
    }
}

#[derive(PartialEq, Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
        assert!(!smaller.can_hold(&larger));
    }
    
    #[test]
    fn assert_partial_equal() {
        let rec1 = Rectangle {
            width: 3,
            height: 3,
        };
        let rec2 = Rectangle {
            width: 3,
            height: 3,
        };
        
        assert_eq!(rec1, rec2);
    }

    #[test]
    #[should_panic(expected = "Guess value must be")] // 실패 메시지에 "Guess value must be"가 포함되어야 함
    fn greater_than_100() {
        Guess::new(200);
    }
    
    #[test]
    #[ignore] // cargo test -- --ignored 명령어로 무시된 테스트만 실행 가능
    fn assert_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
