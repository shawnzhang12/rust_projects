fn main() {
    let number = 3;

    // Condition MUST be a bool (unlike python)
    if number < 5 { 
        println!("Condition was true");
    } else {
        println!("Condiction was false");
    }

    // Consider using match if you have too many else ifs
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // if is an expression
    let condition = true;
    // Both if arm and else arm must have same type
    let number = if condition {5} else {6};
    println!("The value of number is: {number}");
}
