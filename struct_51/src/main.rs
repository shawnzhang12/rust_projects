struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//Tuple Structs, when naming the params is redundant
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//WTF is unit like structs, wild stuff
struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    // some epic struct update syntax
    // cant use user1 as a whole !?, yeah, the username string data is passed now to user2
    // if it were only active and sign_in_count that was copied, then still okay
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

// Field init shorthand since the param and the struct field name have exactly the same names
fn build_user(email: String, username: String) -> User {
    // The params dont have to necessarily come in order
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}