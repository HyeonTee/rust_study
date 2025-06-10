struct DebugDrop(&'static str);

impl Drop for DebugDrop {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("3 hours: {THREE_HOURS_IN_SECONDS}");

    let y = "abcdef";
    println!("y: {y}");

    let y = y.len();
    println!("new y: {y}");
    
    let a: u8 = 255;
    let b = a.checked_add(1);
    match b {
        Some(x) => println!("{x}"),
        None => println!("overflow"),
    }
    
    let quotient = 56.7 / 32.2;
    println!("quotient: {quotient}");
    
    let heart_eyed_cat = '😻';
    println!("heart_eyed_cat: {heart_eyed_cat}");
    
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup: {tup:?}");
    
    let months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    println!("months.len(): {}", months.len());
    
    let arr: [i32; 5] = [3; 5];
    println!("arr[1]: {}", arr[1]);
    
    let unit = println!("returning Unit");
    match unit {
        () => println!("the returned value is Unit"),
    }

    let f = || println!("나는 () → () 클로저");
    f();

    let _x = DebugDrop("first x"); // x₁
    let _x = DebugDrop("second x"); // x₂ → x₁ shadowing

    println!("스코프 끝나기 전");
}
