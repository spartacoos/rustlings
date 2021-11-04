#[derive(Debug)]
struct Cars;

fn main() {
    let ferrari = Cars;
    let sam = Cars {};
    println!("Hello, world!");
    println!("{:?}", Cars);
    println!("{:?}", ferrari);
    println!("{:?}", sam);
}
