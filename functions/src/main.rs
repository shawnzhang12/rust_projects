fn main() {
    println!("Hello, world!");

    another_function(5);

    print_labeled_measurement(5, 'h');

    let _y = { //Expressions return a value, this scope block is an expression
        let x = 3;
        x+1
    };
    let _z = 6_000_000; //This is a statement, unused variable, number with underscore for better reading

    let x = five();
    println!("x is {x}");
}

fn another_function(x: i32) {
    println!("The value of x is {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is {value}{unit_label}");
}

fn five() -> i32 {
    5
}

