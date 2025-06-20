use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // let expensive_closure = |num: u32| -> u32 {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };
    // 
    // let num = expensive_closure(1);
    // println!("num: {}", num);
    
    let explain_closure = |x| x;
    let s = explain_closure(String::from("hello"));
    let n = explain_closure(5.to_string());

    let mut x = 5;

    let mut change_x = || {
        x += 1;
    };

    change_x();
    println!("x is {}", x);
    
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    // let only_borrows = || println!("From closure: {:?}", list);
    let mut borrows_mutably = || list.push(8);
    
    // println!("Before calling closure: {:?}", list);
    // only_borrows();
    borrows_mutably();
    println!("After calling closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
    
    let mut list = [
        Rectangle { width: 2, height: 13 },
        Rectangle { width: 10, height: 27 },
        Rectangle { width: 7, height: 3 },
        Rectangle { width: 44, height: 1 },
    ];
    
    // println!("Before sort by key: {:?}", list);
    // list.sort_by_key(|r| r.width);
    // println!("After sort by key: {:?}", list);

    // let mut sort_operations = vec![];
    // let value = String::from("by key called");
    // 
    // list.sort_by_key(|r| {
    //     sort_operations.push(value);
    //     r.width
    // });
    // println!("{:#?}", list);
    
    let mut num_sort_operations = 0;
    list.sort_by_cached_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}
