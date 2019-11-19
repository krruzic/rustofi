//! This Rust library enables the construction of complex multipage applications that use Rofi to
//! display their UI. The basic idea is to create a `AppPage` or `SearchPage` as an application
//! main menu and feed it in possible selections and actions. These selections and actions can
//! then navigate you to an `ItemList`, an `EntryBox` an `ActionList` or another main menu.
//!
//! Typically you will want to create an AppPage with some options and actions,
//! then display it in a loop checking the return for the `RustofiOptionType` to exit on.
//! `AppPage` and `SearchPage` will automatically add an exit option to simplify the loop exit cases, while
//! `ItemList` and `ActionList` will add a cancel option.
//!
//! # Simplest Possible Example
//!
//! The below example gets even simpler than creating an AppRoot, just displaying a list of strings
//! and utilizing a callback to print the selected item. Notice the loop in main checking the
//! return variant of the rofi window
//!
//! ```no_run
//! use rustofi::components::ItemList;
//! use rustofi::RustofiResult;
//!
//! fn simple_app() -> RustofiResult {
//!     let rustofi_entries = vec![
//!         "Entry 1".to_string(),
//!         "Entry 2".to_string(),
//!         "Entry 3".to_string(),
//!     ];
//!     ItemList::new(rustofi_entries, Box::new(simple_callback)).display("Select an entry".to_string())
//! }
//!
//! pub fn simple_callback(s: &String) -> RustofiResult {
//!     println!("Clicked on item: {}", s);
//!     RustofiResult::Success
//! }
//!
//! fn main() {
//!     loop {
//!         match simple_app() {
//!             RustofiResult::Error => break,
//!             RustofiResult::Exit => break,
//!             RustofiResult::Cancel => break,
//!             RustofiResult::Blank => break,
//!             _ => {}
//!         }
//!     }
//! }
//! ```

/// extra rofi window types usable to create an application, essentially navigation result pages
pub mod components;
/// the error(s) returned by this crate
pub mod errors;
/// raw representation of a rofi command, use this to create new components, or your own from-scratch
/// apps
pub mod window;

use std::clone::Clone;
use std::fmt::Display;

use crate::window::{Dimensions, Location, Window};

/// enum declaring all possible return values from a rofi window constructed
/// using this library. Callbacks should also generally return this type, specifying
/// `Success`, `Error`, `Exit` or `Cancel` in most cases
pub enum RustofiResult {
    /// A standard item
    Selection(String),
    /// An action item
    Action(String),
    /// The operation completed successfully
    Success,
    /// The blank entry was selected. Note this entry isn't actually blank but a single space
    Blank,
    /// Something went wrong creating the rofi window or in the callback
    Error,
    /// `ItemList` or `ActionList` was cancelled, used to return to a main menu
    Cancel,
    /// Used internally when the automatically added `[exit]` entry is selected
    Exit
}

/// Wrapper around a callback that returns a RustofiResult
pub trait RustofiCallback<T>: FnMut(&T) -> RustofiResult {
    fn clone_boxed(&self) -> Box<dyn RustofiCallback<T>>;
}
impl<T, C> RustofiCallback<T> for C
where
    C: 'static + Clone + FnMut(&T) -> RustofiResult
{
    fn clone_boxed(&self) -> Box<dyn RustofiCallback<T>> {
        Box::new(self.clone())
    }
}
impl<T: 'static> Clone for Box<dyn RustofiCallback<T>> {
    fn clone(&self) -> Self {
        self.clone_boxed()
    }
}

/// Trait implemented by `SearchPage` and `AppPage`.
pub trait RustofiComponent<'a> {
    /// returns a rofi window with special initial options for the implementation
    fn create_window() -> Window<'a>;
    /// set the callback associated with actions
    fn action(self, acb: Box<dyn FnMut(&String) -> RustofiResult>) -> Self;
    /// set the callback associated with the blank entry item
    fn blank(self, bcb: Box<dyn FnMut() -> RustofiResult>) -> Self;
    /// set the optional actions to display
    fn actions(self, actions: Vec<String>) -> Self;
    /// customize the implementation's rofi window
    fn window(self, window: Window<'a>) -> Self;
    /// run the rofi command
    fn display(&mut self, prompt: String) -> RustofiResult;
}

/// `AppPage` displays a single column rofi window and is meant to be used as a main menu
/// of sorts for your application. `items` should be associated with a data model, while `actions`
/// should be either operations you can perform on those items, or actions you can take within the
/// app (switch pages for example)
pub struct AppPage<'a, T> {
    /// standard list items, will be displayed in the rofi window using to_string()
    pub items: Vec<T>,
    /// callback called whenever an item in the `items` vector is selected
    pub item_callback: Box<dyn RustofiCallback<T>>,
    /// callback called whenever a blank entry is selected
    pub blank_callback: Box<dyn FnMut() -> RustofiResult>,
    /// additional action entries, meant to be operations on standard items
    pub actions: Vec<String>,
    /// callback called whenever a custom action is selected (NOT on Exit or Cancel)
    pub action_callback: Box<dyn FnMut(&String) -> RustofiResult>,
    /// rofi window instance
    pub window: Window<'a>
}

impl<'a, T: Display + Clone> AppPage<'a, T> {
    /// create the initial bare minumum AppPage, without showing the window yet
    pub fn new(items: Vec<T>, item_callback: Box<dyn RustofiCallback<T>>) -> Self {
        AppPage {
            items,
            item_callback,
            actions: vec![" ".to_string(), "[exit]".to_string()],
            blank_callback: Box::new(|| RustofiResult::Blank),
            action_callback: Box::new(|_| RustofiResult::Action("".to_string())),
            window: SearchPage::<T>::create_window()
        }
    }

    /// A message usually displayed right beneath the prompt in a rofi window. You can
    /// use this to display instructions
    pub fn message(mut self, message: &'static str) -> Self {
        self.window = self.window.message(message);
        self
    }
}

impl<'a, T: Display + Clone> RustofiComponent<'a> for AppPage<'a, T> {
    /// create a centred single column rofi window with Pango markup enabled
    fn create_window() -> Window<'a> {
        Window::new("AppList")
            .format('s')
            .location(Location::MiddleCentre)
            .add_args(vec!["-markup-rows".to_string()])
    }

    /// set the callback to be run when an action is selected
    fn action(mut self, acb: Box<dyn FnMut(&String) -> RustofiResult>) -> Self {
        self.action_callback = acb;
        self
    }

    /// set the callback to be run when the blank entry is selected
    fn blank(mut self, bcb: Box<dyn FnMut() -> RustofiResult>) -> Self {
        self.blank_callback = bcb;
        self
    }

    /// set the actions in the AppPage. This should only be called once as it overwrites
    /// the previous settings
    fn actions(mut self, mut actions: Vec<String>) -> Self {
        actions.insert(0, " ".to_string());
        actions.insert(0, "[exit]".to_string());
        self.actions = actions;
        self
    }

    /// set a completely custom window
    fn window(mut self, window: Window<'a>) -> Self {
        self.window = window.format('s'); // ensure we're in string mode
        self
    }

    /// run the rofi and match the selection to a `RustofiResult`
    fn display(&mut self, prompt: String) -> RustofiResult {
        let mut display_options: Vec<String> = self.items.iter().map(|s| s.to_string()).collect();
        display_options.append(self.actions.as_mut());
        let response = self
            .window
            .clone()
            .prompt(prompt)
            .lines(display_options.len() as i32)
            .show(display_options.clone());
        match response {
            Ok(input) => {
                if input == "[exit]" || input == "" {
                    RustofiResult::Exit
                } else if input == " " {
                    (self.blank_callback)()
                } else {
                    // check if the entry matches one of the list items
                    for item in self.items.clone() {
                        if input == item.to_string() {
                            return (self.item_callback)(&item);
                        }
                    }

                    // check if the entry matches one of the action items
                    for item in self.actions.clone() {
                        if input == item.to_string() {
                            return (self.action_callback)(&input);
                        }
                    }
                    // if the entry isn't an action or an existing entry item, return exit
                    RustofiResult::Exit
                }
            }
            Err(_) => RustofiResult::Error
        }
    }
}

/// `SearchPage` displays a multi column rofi window and is meant to be used as a search page
/// of sorts for your application. `items` should be associated with a data model, while `actions`
/// should be either operations you can perform on those items, or actions you can take within the
/// app (switch pages for example). The `search_callback` allows you to refresh the data models
/// displayed or perform an operation on custom entry
pub struct SearchPage<'a, T> {
    /// standard list items, will be displayed in the rofi window using to_string()
    pub items: Vec<T>,
    /// callback called whenever an item in the `items` vector is selected
    pub item_callback: Box<dyn RustofiCallback<T>>,
    /// callback called whenever a blank entry is selected
    pub blank_callback: Box<dyn FnMut() -> RustofiResult>,
    /// additional action entries, meant to be operations on standard items
    pub actions: Vec<String>,
    /// callback called whenever a custom action is selected (NOT on Exit or Cancel)
    pub action_callback: Box<dyn FnMut(&String) -> RustofiResult>,
    /// callback to be run when no other entry matches
    pub search_callback: Box<dyn FnMut(&String) -> RustofiResult>,
    /// rofi window instance
    pub window: Window<'a>
}

impl<'a, T: Display + Clone> SearchPage<'a, T> {
    /// create the initial bare minumum AppPage, without showing the window yet
    pub fn new(
        items: Vec<T>, item_callback: Box<dyn RustofiCallback<T>>,
        search_callback: Box<dyn FnMut(&String) -> RustofiResult>
    ) -> Self {
        SearchPage {
            items,
            item_callback,
            actions: vec![" ".to_string(), "[cancel]".to_string()],
            blank_callback: Box::new(|| RustofiResult::Blank),
            action_callback: Box::new(|_| RustofiResult::Action("".to_string())),
            search_callback,
            window: SearchPage::<T>::create_window()
        }
    }
}

impl<'a, T: Display + Clone> RustofiComponent<'a> for SearchPage<'a, T> {
    /// create a rofi window with 4 columns
    fn create_window() -> Window<'a> {
        Window::new("Search")
            .format('s')
            .location(Location::MiddleCentre)
            .dimensions(Dimensions {
                width: 640,
                height: 480,
                lines: 5,
                columns: 4
            })
            .add_args(vec!["-markup-rows".to_string()])
    }

    /// set the callback to be run when an action is selected
    fn action(mut self, acb: Box<dyn FnMut(&String) -> RustofiResult>) -> Self {
        self.action_callback = acb;
        self
    }

    /// set the callback to be run when the blank entry is selected
    fn blank(mut self, bcb: Box<dyn FnMut() -> RustofiResult>) -> Self {
        self.blank_callback = bcb;
        self
    }

    /// set the actions in the AppPage. This should only be called once as it overwrites
    /// the previous settings
    fn actions(mut self, mut actions: Vec<String>) -> Self {
        actions.insert(0, " ".to_string());
        actions.insert(0, "[exit]".to_string());
        self.actions = actions;
        self
    }

    /// set a completely custom window
    fn window(mut self, window: Window<'a>) -> Self {
        self.window = window.format('s'); // ensure we're in string mode
        self
    }

    /// display the search window and match the entry against the actions, standard items
    /// and finally if nothing matches, run the search callback
    fn display(&mut self, prompt: String) -> RustofiResult {
        let mut display_options: Vec<String> = self.items.iter().map(|s| s.to_string()).collect();
        display_options.append(self.actions.as_mut());
        let response = self
            .window
            .clone()
            .prompt(prompt)
            .show(display_options.clone());
        match response {
            Ok(input) => {
                if input == "[exit]" {
                    RustofiResult::Exit
                } else if input == " " {
                    (self.blank_callback)()
                } else if input == "" {
                    RustofiResult::Cancel
                } else {
                    // check if the entry matches one of the list items
                    for item in self.items.clone() {
                        if input == item.to_string() {
                            return (self.item_callback)(&item);
                        }
                    }

                    // check if the entry matches one of the action items
                    for item in self.actions.clone() {
                        if input == item.to_string() {
                            return (self.action_callback)(&input);
                        }
                    }
                    // if the entry isn't an action or an existing entry item,
                    // run the search callback
                    (self.search_callback)(&input)
                }
            }
            Err(_) => RustofiResult::Error
        }
    }
}
