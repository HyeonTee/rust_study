use my_crate::mix;
use my_crate::PrimaryColor;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    let mixed = mix(red, yellow);
    
    println!("Red + Yellow = {:?}", mixed);
}
