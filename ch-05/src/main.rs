use std::io;

#[derive(Debug)]
struct Rectangle {
    length:u32,
    breadth:u32
}

impl Rectangle  {
    fn area(&self) -> u32 {
        self.length.overflowing_mul(self.breadth).0
    }

    fn fits_in(&self, rectangle: &Rectangle) -> bool {
        self.length > rectangle.length && self.breadth > rectangle.breadth
    }
}

fn main() {
    let rect = Rectangle { length: 50, breadth: 20 };
    println!("Enter length and breadth of rectangle: ");
    let mut l = String::new();
    io::stdin().read_line(&mut l).expect("Failed to read line");
    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Failed to read line");
    let rectangle = Rectangle {
        length: l.trim().parse().expect("Not a number"),
        breadth: b.trim().parse().expect("Not a number")
    };
    if rect.fits_in(&rectangle) {
        println!("Rectangle fits in the first rectangle");
    } else {
        println!("Rectangle does not fit in the first rectangle");
    }
    dbg!(&rectangle);
}
