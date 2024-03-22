// Chapter 7, Crates, Packages, Modules and More

mod front_of_house { 
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

mod test_house;

fn deliver_order() {}

mod back_of_house {
    // all variants automatically public with pub keyword on enum
    pub enum Appetizer {
        Soup,
        Salad,
    }


    // Customer can choose the toast, but the chef decides the fruit
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }


    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

// only for this scope, cannot be used inside other mods like this, can use super::hosting within another module to get the parent scope
use crate::front_of_house::hosting;
// for structs, enums, and others with use, bring in full path
use std::collections:HashMap;


// exception when bringing same name into scope
use std::fmt;
use std::io;
// OR use keywords

use std::fmt::Result;
use std::io::Result as IoResult;

// Re-exporting
// others can also bring hosting into their scope too with pub use
// restaurant::hosting:add_to_waitlist() is now valid
pub use crate::front_of_house::hosting;

// nest paths
use std::{cmp::Ordering, io};
// using self
use std::io::{self, Write};
// bring all public items with glob, best used with tests
use std::collections::*;


fn function1() -> fmt::Result {
    // snip
}

fn function2() -> io::Result<()> {
    // snip
}



// siblings can refer to each other
pub fn eat_at_restaurant() {
    // this is the idiomatic way to use Paths for functions, to show it's imported and not locally defined
    // hosting::add_to_waitlist()

    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // we arent allowed to see or modify the seasonal fruit, it's a suprise :)

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}