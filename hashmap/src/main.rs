use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // All keys must be the same type, as are the values
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // ownership of strings will be moved to the hashmaps if the key or val was a variable

    // accessing values
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }    

    // Overwriting a value
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    // Adding key and value only if key isnt present
    scores.entry(String::from("Yellow")).or_insert(50);

    println!("{:?}", scores);

    //Updating value based on old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    // default is siphash, which is a slower hashing function but provides better security, can switch if needed
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // returns mutable reference
        *count += 1;
    }

}
