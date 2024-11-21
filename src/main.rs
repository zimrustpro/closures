fn main() {
    let example1 = || println!("This is a closure");
    example1();

    let example2 = |x: i32| println!("{}", x);
    example2(5);
    example2(5 + 5);
}
