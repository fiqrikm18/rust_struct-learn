struct Rectangle {
    width: u32,
    height: u32,
}

// note: that if the function in impl block have parameter self that mean
//  the function is associated with the struct and can called with `.` operator,
// in other if the function inside the impl block doesn't have self as parameter
// the function is not associated with the struct and can called with `::` operator.
impl Rectangle {
    // associated function
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // associated function
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // unassociated function
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size } 
    }
}

fn main() {
    let rect1 = Rectangle{
        width: 30, 
        height: 50,
    };

    let rect2 = Rectangle{
        width: 10, 
        height: 40,
    };

    let rect3 = Rectangle{
        width: 60, 
        height: 45,
    };

    let rect4 = Rectangle::square(30);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
