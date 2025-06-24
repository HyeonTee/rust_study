use cons_list::List;

fn main() {
    let list = List::Cons(3, Box::new(List::Cons(2, Box::new(List::Cons(1, Box::new(List::Nil))))));
    
    list.print();
    
    println!("Length: {}", list.length());
    
    println!("Sum: {}", list.sum());
    
    println!("Contains 2? {}", list.contains(2));
    println!("Contains 99? {}", list.contains(99));
    
    let pushed = list.push(10);
    pushed.print();
    
    let popped = pushed.pop();
    popped.print();
}
