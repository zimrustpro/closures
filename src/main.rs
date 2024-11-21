fn main() {
    let example1 = || println!("This is a closure");
    example1();

    let example2 = |x: i32| println!("{}", x);
    example2(5);
    example2(5 + 5);

    let example3 = || {
        let num1 = 7;
        let num2 = 10;
        println!("The two numbers are {} and {}.", num1, num2);
    };
    example3();

    let number1 = 6;
    let number2 = 10;
    let example4 = || println!("{}", number1 + number2);
    example4();
}
