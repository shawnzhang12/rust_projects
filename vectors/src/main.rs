fn main() {
    // creating a new vector
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    println!("Hello, world!");

    // updating a vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // reading vectors
    // 1st option useful when you want the program to crash if invalid indexing
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // Other choice is error handling
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // iterating over values
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    // enum to store multiple types in a vector
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];


}
