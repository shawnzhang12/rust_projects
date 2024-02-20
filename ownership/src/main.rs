fn main() {
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("Hello");
    let s2 = s1; // This copies the 'pointer'/len/capacity of s1 into s2, which means there is only one datum in the heap
    // to prevent a double free, basically when drop is called once the scope is over, s1 is rendered invalid. Only ONE pointer may be allowed to exist in this scope

    // invalid line, since s1 was moved to s2. Not a shallow copy, but a MOVE.
    // println!("{}", s1);
    println!("{}", s2);


    let s1 = String::from("hello");
    let s2 = s1.clone(); // This is okay, since it creates another copy of the data on the heap

    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y); // This is fine too, since for stack data shallow and deep copies are the same

    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward


    let _s1 = gives_ownership();         // gives_ownership moves its return
    // value into s1

    let _s2 = String::from("hello");     // s2 comes into scope

    let _s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3

    // Ownership is annoying if you have to return the parameter as a tuple along with other values if you want to use it again, so pass by references exist, 
    // where ownership is not moved


} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

// Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
// return value into the function
// that calls it
    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
        // moves out to the calling
        // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
             // scope
    a_string  // a_string is returned and moves out to the calling function
}