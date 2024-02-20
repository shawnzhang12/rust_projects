fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);  // Lets us reference the value of s1, without taking ownership
    println!("The length of '{}' is {}.", s1, len);
    // You cannot modify a reference's value. references are immutable.
    
    // unless you make a mutable string and mutable reference
    let mut s = String::from("hello");
    let _r1 = &mut s;
    // This is not allowed, only one mutable reference per value, per scope
    // Prevents data races
    // let r2 = &mut s;
    change(&mut s);

    let mut ss = String::from("hello");
    let r1 = &ss; // no problem
    let r2 = &ss; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    // we cannot have a mutable reference while also having an immutable reference
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be sude after this point
    let r3 = &mut ss; // no problem now
    println!("{}", r3);

}    

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of 
  // what it refers to, it is not dropped.

fn dangle() -> &String {
    len s = String::from("hello"); // s is a new String
    &s //we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away. Danger!

// RECAP
// 1. At any given time, you can have 'either' 
// one mutable reference or any number of immutable references
// 2. References must always be valid.