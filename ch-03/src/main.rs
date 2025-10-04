fn main() {
    let x: u8 = 255;
    let mut y = x.wrapping_add(2);
    println!("x = {}, y = {}", x, y);
    y = x.checked_add(2).unwrap_or(0);
    println!("y = {:?}", y);
    y = x.saturating_add(2);
    println!("y = {}", y);
    y = x.overflowing_add(2).0;
    println!("y = {}", y);
}
