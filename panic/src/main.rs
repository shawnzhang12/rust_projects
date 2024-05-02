use std::fs::File;
use std::io::{self, Read};
use std::io::ErrorKind;
use std::fs;

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn main() {
    //panic!("crash and burn");
    let v = vec![1, 2, 3];

    //v[99];
    //RUST_BACKTRACE=1 cargo run, find first line of code that user wrote

    // File::open is a Result<T, E>
    // returns std::fs::File if Ok else std::io:Error if Err
    let greeting_file_result = File::open("hello.txt");

    // use match to handle cases
    // Ok, Err, brought in by Result<T, E>
    // we can match on different error, and handle each case accordingly
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // unwrap is a shortcut that returns the ok value if Ok, else calls panic if there's an error
    let greeting_file = File::open("hello.txt").unwrap();

    // expect just like unwrap but with custom error message
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");

}

// Propagating the long way
fn read_username_from_file() -> Result<String, std::io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// Question mark to provide same functionality, just returns Error if it hits
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// Chaining method calls
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

// The used approach, the fs crate does this already
fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

//? can be used on strictly either Result or Option, no mixing and matching, need OK or OK_or to explicitly convert between the two

// fn main() -> Result<(), Box<dyn Error>> {
//     let greeting_file = File::open("hello.txt")?;

//     Ok(())
// }

//Examples, Prototypes, Tests --> panic with unwrap or expect, you want it to fail
//Occasional expected failures like user passing in wrong type --> Err and Result