fn main() {
    let v = vec![1,2,3];
    let x = v.get(4);
    match x {
        Some(x) => println!("Found: {x}"),
        None => println!("Not found"),
    }
}