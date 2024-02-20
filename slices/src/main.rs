fn main() {
    let s = String::from("hello world");

    // doesnt include the end index, like python
    let hello = &s[0..5];
    let world = &s[6..11];
    let world = &s[6..]; // same thing if include last byte
    let word = first_word_slice(&s);
    //s.clear(); //error, word is an immutable reference to s
    // clear needs to truncate the string, so it is a mutable reference
    println!("the first word is: {}", word); // here is the immutable reference to s
    // we CANT have an immutable AND mutable reference at the same time, hence error

    let s = "Hello, world!"; //string literal, type is &str, immutable reference

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3])
    // slice has type &[i32], stores first element and length
}

// without slices, since we dont need ownership, reference is fine
// problem is we are returning a usize, which isnt connected to s as all!
// if s were to clear later, our return value would mean nothing
// signature &str accepts both &String and &str
fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}