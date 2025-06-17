// 제네릭 타입은 컴파일 타임에 단형성(monomorphism)화, 런타임 비용 발생 X
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
    
    fn y(&self) -> &U {
        &self.y
    }
    
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// impl Point<f32, f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//     
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     
//     largest
// }

fn main() {
    // let number_list = vec![34, 50, 25, 100, 65];
    // 
    // let result = largest(&number_list);
    // println!("The largest number is {}", result);
    // 
    // let char_list = vec!['y', 'm', 'a', 'q'];
    // 
    // let result = largest(&char_list);
    // println!("The largest char is {}", result);
    
    let point_1 = Point { x: 5.1, y: 10.4 };
    let point_2 = Point { x: "Hello", y: 'c' };
    // println!("x: {}, y: {}", point_1.x(), point_1.y());
    // println!("x: {}, y: {}", point_2.x(), point_2.y());
    
    // println!("distance_from_origin: {}", point_1.distance_from_origin());
    // println!("distance_from_origin: {}", point_2.distance_from_origin());
    
    let point_3 = point_2.mixup(point_1);
    println!("x: {}, y: {}", point_3.x, point_3.y);
}