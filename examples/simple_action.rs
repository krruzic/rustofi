// examples/simple_action.rs
use std::io::{Error, ErrorKind};
use std::result::Result;

use rustofi::components::ActionList;
use rustofi::CallbackResult;
use rustofi::RustofiResult;

// notice the Clone derive and Display implementation? These are
// necessary if you want to pass in a custom type!
// Otherwise you'll get an error like this
// error[E0277]: the trait bound `Person: std::clone::Clone` is not satisfied
#[derive(Clone)]
pub struct Person {
    pub age: i32,
    pub name: String
}
impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name) // just display the name when calling to_string()
    }
}
fn simple_app(person: Person) -> RustofiResult {
    // create two actions, one that simulates an age increase, one a decrease
    let rustofi_entries = vec!["Age Up".to_string(), "Age Down".to_string()];
    // construct the ActionList and immediately display it with a prompt
    // showing which person is being modified
    ActionList::new(person.clone(), rustofi_entries, Box::new(simple_callback))
        .display(format!("looking at {}, age {}", person.name, person.age))
}

pub fn simple_callback(person: &Person, action: &String) -> CallbackResult {
    println!("selected action: {}", action);
    // match which action was selected
    if action == "Age Up" {
        println!("{} age + 5 is: {} ", person.name, person.age + 5);
    } else if action == "Age Down" {
        println!("{} age - 5 is: {}", person.name, person.age - 5);
    } else {
        // user entered a custom string
        println!("invalid action!");
        return Err("invalid action".to_string());
    }
    return Ok(());
}

fn main() {
    let mut p = Person {
        age: 15,
        name: "joe".to_string()
    };
    loop {
        match simple_app(p.clone()) {
            // loop until an exit or error occurs
            RustofiResult::Error(_) => break,
            RustofiResult::Exit => break,
            RustofiResult::Cancel => break,
            RustofiResult::Blank => break, // we could give the blank entry special powers
            _ => {}
        }
    }
}
