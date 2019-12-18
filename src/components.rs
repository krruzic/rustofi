//! `ItemList`, `ActionList` and `EntryBox` are additional components or controls you can use to build
//! your application.
//!
//! # Examples
//!
//! ## Using ItemList
//! This example demonstrates using the ItemList by creating a list of strings then printing off which
//! was selected. Run with:
//!
//! `cargo run --example simple`
//! ```no_run
//! // examples/simple.rs
//! use rustofi::components::ItemList;
//! use rustofi::RustofiResult;
//!
//! fn simple_app() -> RustofiResult {
//!     // create a list of strings to pass as rofi options. Note that this can be any type you want,
//!     // the callback will always return the type you passed in
//!     let rustofi_entries = vec![
//!         "Entry 1".to_string(),
//!         "Entry 2".to_string(),
//!         "Entry 3".to_string(),
//!     ];
//!     // create a ItemList with a callback that prints which item was selected.
//!     ItemList::new(rustofi_entries, Box::new(simple_callback)).display("Select an entry".to_string())
//! }
//!
//! pub fn simple_callback(s: &String) -> RustofiResult {
//!     // when an item is clicked, print the name!
//!     println!("Clicked on item: {}", s);
//!     RustofiResult::Success
//! }
//!
//! fn main() {
//!     loop {
//!         match simple_app() {
//!             //!  loop unless the user requests we exit
//!             RustofiResult::Error => break,
//!             RustofiResult::Exit => break,
//!             RustofiResult::Cancel => break,
//!             RustofiResult::Blank => break,
//!             _ => {}
//!         }
//!     }
//! }
//! ```
//! ## Using ActionList
//! This example demonstrates using the ActionList to manipulate an object's state. As it can't
//! return a modified item through the callback, you'll need to store your modified changes with
//! real storage or a global variable of some sort. In this example the data is only temporary.
//! Run with:
//!
//! `cargo run --example simple_action`
//! ```no_run
//! // examples/simple_action.rs
//! use rustofi::components::ActionList;
//! use rustofi::RustofiResult;
//!
//! // notice the Clone derive and Display implementation? These are
//! // necessary if you want to pass in a custom type!
//! // Otherwise you'll get an error like this
//! // error[E0277]: the trait bound `Person: std::clone::Clone` is not satisfied
//! #[derive(Clone)]
//! pub struct Person {
//!     pub age: i32,
//!     pub name: String
//! }
//! impl std::fmt::Display for Person {
//!     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//!         write!(f, "{}", self.name) // just display the name when calling to_string()
//!     }
//! }
//! fn simple_app(person: Person) -> RustofiResult {
//!     // create two actions, one that simulates an age increase, one a decrease
//!     let rustofi_entries = vec!["Age Up".to_string(), "Age Down".to_string()];
//!     // construct the ActionList and immediately display it with a prompt
//!     // showing which person is being modified
//!     ActionList::new(person.clone(), rustofi_entries, Box::new(simple_callback))
//!         .display(format!("looking at {}, age {}", person.name, person.age))
//! }
//!
//! pub fn simple_callback(person: &Person, action: &String) -> RustofiResult {
//!     println!("selected action: {}", action);
//!     // match which action was selected
//!     if action == "Age Up" {
//!         println!("{} age + 5 is: {} ", person.name, person.age);
//!     } else if action == "Age Down" {
//!         println!("{} age - 5 is: {}", person.name, person.age);
//!     } else { // user entered a custom string
//!         println!("invalid action!");
//!         return RustofiResult::Error;
//!     }
//!     RustofiResult::Success
//! }
//!
//! fn main() {
//!     let mut p = Person {
//!         age: 15,
//!         name: "joe".to_string()
//!     };
//!     loop {
//!         match simple_app(p.clone()) {
//!             // loop until an exit or error occurs
//!             RustofiResult::Error => break,
//!             RustofiResult::Exit => break,
//!             RustofiResult::Cancel => break,
//!             RustofiResult::Blank => break, // we could give the blank entry special powers
//!             _ => {}
//!         }
//!     }
//! }
//! ```
use std::clone::Clone;
use std::fmt::Display;

use crate::window::{Location, Window};
use crate::{CallbackResult, RustofiCallback, RustofiResult};

/// `ItemList` is a simple rofi window with a selection of items backed by a type `T`. Each item
/// runs the same callback.
pub struct ItemList<'a, T> {
    pub items: Vec<T>,
    pub item_callback: Box<dyn RustofiCallback<T>>,
    pub window: Window<'a>
}

impl<'a, T: Display + Clone> ItemList<'a, T> {
    /// create a new ItemList with the given items and callback
    pub fn new(items: Vec<T>, item_callback: Box<dyn RustofiCallback<T>>) -> Self {
        ItemList {
            items,
            item_callback,
            window: ItemList::<T>::create_window()
        }
    }

    /// create a simple rofi instance representing a window in the middle of the screen
    fn create_window() -> Window<'a> {
        Window::new("ItemList")
            .format('s')
            .location(Location::MiddleCentre)
            .add_args(vec!["-markup-rows".to_string()])
    }

    /// set a completely custom window
    pub fn window(mut self, window: Window<'a>) -> Self {
        self.window = window.format('s');
        self
    }

    /// run the constructed rofi command and match the output: Calling the specified callback with
    /// selected item `T` or returning `Cancel`, `Blank` or `Error`. If the user's entry isn't in
    /// the list, we return the string back wrapped in a `RustofiResult::Selection`
    pub fn display(&mut self, prompt: String) -> RustofiResult {
        let extra = vec!["".to_string(), "[cancel]".to_string()];
        let mut display_options: Vec<String> = self.items.iter().map(|s| s.to_string()).collect();
        display_options = display_options.into_iter().chain(extra.clone()).collect();
        let response = self
            .window
            .clone()
            .lines(display_options.len() as i32)
            .prompt(prompt)
            .show(display_options.clone());
        match response {
            Ok(input) => {
                if input == "[cancel]" || input == "" {
                    RustofiResult::Cancel
                } else if input == " " {
                    RustofiResult::Blank
                } else {
                    for mut item in self.items.clone() {
                        if input == item.to_string() {
                            match (self.item_callback)(&mut item) {
                                Ok(_) => return RustofiResult::Selection(input),
                                Err(m) => return RustofiResult::Error(m)
                            }
                        }
                    }
                    RustofiResult::Selection(input)
                }
            }
            Err(_) => RustofiResult::Error("error getting user input from rofi".to_string())
        }
    }
}

/// `ActionList` is a simple rofi window with a selection of strings that operate on a
/// single item `T`. When a selection is made, the `action_callback` is called with the item and
/// action name passed as arguments
///
pub struct ActionList<'a, T> {
    pub item: T,
    pub actions: Vec<String>,
    pub action_callback: Box<dyn FnMut(&T, &String) -> CallbackResult>,
    pub window: Window<'a>
}

impl<'a, T: Display + Clone> ActionList<'a, T> {
    /// create a new `ActionList` with an item to operate on, a list of strings representing actions
    /// and a callback to run on selection
    pub fn new(
        item: T, actions: Vec<String>,
        action_callback: Box<dyn FnMut(&T, &String) -> CallbackResult>
    ) -> Self {
        ActionList {
            item,
            actions,
            action_callback,
            window: ActionList::<T>::create_window()
        }
    }

    /// create a simple rofi instance representing a window in the middle of the screen
    fn create_window() -> Window<'a> {
        Window::new("ActionList")
            .format('s')
            .location(Location::MiddleCentre)
            .add_args(vec!["-markup-rows".to_string()])
    }

    /// set a completely custom rofi window
    pub fn window(mut self, window: Window<'a>) -> Self {
        self.window = window.format('s');
        self
    }

    /// run the constructed rofi command and display the window, parsing the selection result
    /// In the case of an empty entry (user exited program most likely) or the cancel entry being
    /// selected we return `RustofiResult::Cancel` and `RustofiResult::Blank` respectively. In all
    /// other cases, we attempt to run the given callback on the action. In the case the entry does
    /// not match the action, we simply return the input wrapped in a `RustofiResult::Action`
    pub fn display(&mut self, prompt: String) -> RustofiResult {
        let extra = vec!["".to_string(), "[cancel]".to_string()];
        let mut display_options: Vec<String> = self.actions.iter().map(|s| s.to_string()).collect();
        display_options = display_options.into_iter().chain(extra.clone()).collect();
        let response = self
            .window
            .clone()
            .lines(display_options.len() as i32)
            .prompt(prompt)
            .show(display_options.clone());
        match response {
            Ok(input) => {
                if input == "[cancel]" || input == "" {
                    RustofiResult::Cancel
                } else if input == " " {
                    RustofiResult::Blank
                } else {
                    for action in self.actions.clone() {
                        if input == action.to_string() {
                            match (self.action_callback)(&self.item, &action.to_string()) {
                                Ok(_) => return RustofiResult::Action(input),
                                Err(m) => return RustofiResult::Error(m)
                            }
                        }
                    }
                    RustofiResult::Action(input)
                }
            }
            Err(_) => RustofiResult::Error("error getting user input from rofi".to_string())
        }
    }
}

/// empty struct representing a rofi window used to take and return user input as a string
pub struct EntryBox {}

impl<'a> EntryBox {
    /// create a rofi window with 0 lines. This is important as it simulates a text entry field
    pub fn create_window() -> Window<'a> {
        Window::new("EntryBox").lines(0).format('s')
    }

    /// run the constructed rofi window and return the user input as a string wrapped in a
    /// `RustofiResult::Selection`
    pub fn display(prompt: String) -> RustofiResult {
        let result = EntryBox::create_window()
            .prompt(prompt)
            .show(vec!["".to_string()]);
        match result {
            Ok(input) => {
                if input == "" {
                    RustofiResult::Cancel
                } else {
                    RustofiResult::Selection(input)
                }
            }
            Err(_) => RustofiResult::Error("error getting user input from rofi".to_string())
        }
    }
}
