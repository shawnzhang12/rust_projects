fn main() {
    // Variables
    let x = 5;
    println!("The value of x is: {x}");

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is : {x}");
    }

    println!("The value of x is : {x}");

    let spaces = "   ";
    let spaces = spaces.len();

    println!("The number of spaces is : {spaces}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("A constant : {THREE_HOURS_IN_SECONDS}");

    // Data Types

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    let t = true;

    let f: bool = false; // with explicit type annotation

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    let a = [1, 2, 3, 4, 5];

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
