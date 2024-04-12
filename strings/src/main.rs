fn main() {
    let mut s = String::new();

    let data = "initial contents";

    // all equivalent
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");
    let hello = String::from("你好");

    println!("Hello, world!");

    // Updating a string
    let mut s = String::from("foo");
    // takes in string slice, so does not take ownership (in this case 'bar')
    s.push_str("bar");
    // push takes only one character
    let mut s = String::from("lo");
    s.push('l');

    // concatenation
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // add takes ownership of self, so s1 is no longer valid after
    let s3 = s1 + &s2; // s1 can no longer be used, s2 still valid though (String vs &str?, coerced)

    //use format for unwieldy concatenations, doesnt take ownership
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    // Rust doesnt support string indexing, some unicode values are more than 1 byte
    // Bytes, scalar values, grapheme clusters
    // some wild stuff, just remember that rust doesnt support string indexing

    // String slicing with bytes
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // 4 bytes so Зд since each is 2 bytes for this case, be careful with slicing

    // be explicit when wanting characters or bytes
    for c in "Зд".chars() {
        println!("{c}");
    } // prints З then д

    for b in "Зд".bytes() {
        println!("{b}");
    } // prints 208, 151, 208, 180

}

