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

    (1..=3).for_each(|num| println!("{}", num));
    (1..=3).for_each(|num| {
        println!("Got a {}!", num);
        if num % 2 == 0 {
            println!("It's even");
        } else {
            println!("It's odd");
        }
    });

    let my_vec = vec![8, 9, 10];
    let fourth = my_vec.get(3).unwrap_or_else(|| {
        if let Some(val) = my_vec.get(2) {
            val
        } else {
            &0
        }
    });
    println!("{}", fourth);

    let char_vec = vec!['z', 'y', 'x'];
    char_vec
        .iter()
        .enumerate() // now each item is (usize, char) instead of just char
        .for_each(|(index, c)| println!("Index {} is: {}", index, c));
}
