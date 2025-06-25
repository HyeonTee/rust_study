use crate::List::{Cons, Nil};
use std::rc::Rc;

pub enum List {
    Cons(i32, Rc<List>),
    Nil,
}

impl List {
    pub fn print(&self) {
        match self {
            Cons(val, next) => {
                print!("{} -> ", val);
                next.print();
            }
            Nil => {
                println!("Nil");
            }
        }
    }
    
    pub fn head(&self) -> Option<&i32> {
        match self {
            Cons(val, _) => Some(val),
            Nil => None,
        }
    }
    
    pub fn tail(&self) -> Option<&List> {
        match self {
            List::Cons(_, next) => Some(next),
            List::Nil => None,
        }
    }
    
    pub fn length(&self) -> usize {
        match self {
            List::Cons(_, next) => 1 + next.length(),
            List::Nil => 0,
        }
    }

    pub fn sum(&self) -> i32 {
        match self {
            List::Cons(val, next) => *val + next.sum(),
            List::Nil => 0,
        }
    }

    pub fn contains(&self, target: i32) -> bool {
        match self {
            List::Cons(val, next) => *val == target || next.contains(target),
            List::Nil => false,
        }
    }

    pub fn push(self, val: i32) -> List {
        List::Cons(val, Rc::new(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_list() -> List {
        List::Cons(3, Rc::new(List::Cons(2, Rc::new(List::Cons(1, Rc::new(List::Nil))))))
    }

    #[test]
    fn test_print() {
        let list = sample_list();
        list.print(); // 수동 확인용: 3 -> 2 -> 1 -> Nil
    }

    #[test]
    fn test_head() {
        let list = sample_list();
        assert_eq!(list.head(), Some(&3));
    }

    #[test]
    fn test_tail() {
        let list = sample_list();
        let tail = list.tail().unwrap();
        assert_eq!(tail.head(), Some(&2));
    }

    #[test]
    fn test_length() {
        let list = sample_list();
        assert_eq!(list.length(), 3);
    }

    #[test]
    fn test_sum() {
        let list = sample_list();
        assert_eq!(list.sum(), 6);
    }

    #[test]
    fn test_contains() {
        let list = sample_list();
        assert!(list.contains(2));
        assert!(!list.contains(99));
    }

    #[test]
    fn test_push() {
        let list = sample_list();
        let new_list = list.push(10);
        assert_eq!(new_list.head(), Some(&10));
        assert_eq!(new_list.length(), 4);
    }
}

