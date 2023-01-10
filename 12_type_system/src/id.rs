fn id<T>(x: T) -> T {
    return x;
}
fn main() {
    let int = id(10);
    let string = id("Air");
    println!("{}, {}", int, string);
}
