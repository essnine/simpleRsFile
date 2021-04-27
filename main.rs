fn main() {
    let a_number = 10;
    println!("the number is {}.", a_number);
    let a_boolean = true;
    println!("the boolean is {}.", a_boolean);

    let number = 5;
    let number = number + 5;
    let number = number * 2;
    println!("the number is {}.", number);

    let number: u32 = "42".parse().expect("Not a number");
    println!("the number is {}.", number);
    
    println!("1+2={}", 1u32 + 2);
    println!("1-2={}", 1i32 - 2);
    println!("9/2={}", 9u32 / 2);
    println!("9/2={}", 9.0 / 2.0);
    println!("3*6={}", 3*6);

    let is_bigger = 1 > 4;
    println!("{}", is_bigger); // prints false
}