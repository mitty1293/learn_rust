pub fn fizz_buzz3() {
    for x in 1 .. 51 {
        match x % 15 {
            0 => println!("FizzBuzz"),
            3|6|9|12 => println!("Fizz"),
            5|10 => println!("Buzz"),
            _ => println!("{}", x),
        }
    }
}