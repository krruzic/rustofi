use std::clone::Clone;
use std::fmt::Display;

use crate::window::{Location, Window};
use crate::{RustofiResult, RustofiCallback};

pub struct ItemList<'a, T> {
    pub items: Vec<T>,
    pub item_callback: Box<dyn RustofiCallback<T>>,
    pub window: Window<'a>
}

impl<'a, T: Display + Clone> ItemList<'a, T> {
    pub fn new(items: Vec<T>, item_callback: Box<dyn RustofiCallback<T>>) -> Self {
        ItemList {
            items,
            item_callback,
            window: ItemList::<T>::create_window()
        }
    }

    fn create_window() -> Window<'a> {
        Window::new("ItemList")
            .format('s')
            .location(Location::MiddleCentre)
            .add_args(vec!["-markup-rows".to_string()])
    }

    pub fn window(mut self, window: Window<'a>) -> Self {
        self.window = window;
        self
    }

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
                    for item in self.items.clone() {
                        if input == item.to_string() {
                            return (self.item_callback)(&item);
                        }
                    }
                    RustofiResult::Selection(input)
                }
            }
            Err(_) => RustofiResult::Error
        }
    }
}

pub struct ActionList<'a, T> {
    pub item: T,
    pub actions: Vec<String>,
    pub action_callback: Box<dyn FnMut(&T, &String) -> RustofiResult>,
    pub window: Window<'a>
}

impl<'a, T: Display + Clone> ActionList<'a, T> {
    pub fn new(
        item: T, actions: Vec<String>,
        action_callback: Box<dyn FnMut(&T, &String) -> RustofiResult>
    ) -> Self {
        ActionList {
            item,
            actions,
            action_callback,
            window: ActionList::<T>::create_window()
        }
    }

    fn create_window() -> Window<'a> {
        Window::new("ActionList")
            .format('s')
            .location(Location::MiddleCentre)
            .add_args(vec!["-markup-rows".to_string()])
    }

    pub fn window(mut self, window: Window<'a>) -> Self {
        self.window = window;
        self
    }

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
                            return (self.action_callback)(&self.item, &action.to_string());
                        }
                    }
                    RustofiResult::Action(input)
                }
            }
            Err(_) => RustofiResult::Error
        }
    }
}

pub struct EntryBox {}

impl<'a> EntryBox {
    pub fn create_window() -> Window<'a> {
        Window::new("EntryBox").lines(0).format('s')
    }

    pub fn new(prompt: String) -> RustofiResult {
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
            Err(_) => RustofiResult::Error
        }
    }
}
