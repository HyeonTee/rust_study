#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn perimeter(self: &Self) -> u32 { // &self는 이것의 축약
        2 * (self.width + self.height)
    }
    
    fn resize_width(&mut self, width: u32) {
        self.width = width;
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width >= other.width && self.height >= other.height) || (self.width >= other.height && self.height >= other.width)
    }
    
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // let width1 = 30;
    // let height1 = 50;
    
    // let rect1 = (30, 50);
    
    let mut rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!(
        "The area of the rectangle is {} square pixels.",
        area_statement(&rect2) // 참조자로 넘겨주지 않으면 area_statement로 소유권 넘어가서 이후 사용 불가
    );
    
    println!(
        "The area of the rectangle is {} square pixels.",
        rect2.area()
    );
    
    println!("The perimeter of the rectangle is {}", rect2.perimeter());
    
    rect2.resize_width(60);
    
    println!("The resized area of the rectangle is {}", rect2.area());
    
    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    
    dbg!(&rect3);
    
    dbg!(&rect2.can_hold(&rect3));
    
    let square = Rectangle::square(70);
    dbg!(&rect3.can_hold(&square));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}

fn area_statement(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}