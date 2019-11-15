mod errors;
pub mod window;

use crate::window::*;

use std::str;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum RustofiOptionType {
    Selection,
    Action,
    Blank,
    Exit
}

pub struct RustofiOption {
    pub display: String,
    pub callback: Box<dyn FnMut(&str)>,
    pub option: RustofiOptionType
}

impl RustofiOption {
    fn run(&mut self) {
        (self.callback)(&self.display);
    }
}

pub struct AppRoot {
    pub selections: Vec<RustofiOption>,
    pub actions: Vec<RustofiOption>,
    pub response: RustofiOptionType
}

impl AppRoot {
    fn calculate_blanks(selection_len: i32, columns: i32) -> Vec<RustofiOption> {
        let num_blanks = selection_len % columns;
        let mut blanks = Vec::new();
        for _x in 0..num_blanks + columns {
            blanks.push(RustofiOption {
                display: "".to_string(),
                callback: Box::new(AppRoot::nop),
                option: RustofiOptionType::Blank
            })
        }
        blanks
    }
}

impl AppRoot {
    fn nop(_s: &str) {}

    pub fn new(entries: Vec<RustofiOption>) -> Self {
        let mut combolist = AppRoot {
            selections: Vec::new(),
            actions: Vec::new(),
            response: RustofiOptionType::Exit
        };
        let exit_action = RustofiOption {
            display: "[Exit]".to_string(),
            callback: Box::new(AppRoot::nop),
            option: RustofiOptionType::Exit
        };
        for entry in entries {
            match entry.option {
                RustofiOptionType::Selection => combolist.selections.push(entry),
                RustofiOptionType::Action => combolist.actions.push(entry),
                _ => {}
            }
        }
        combolist.actions.push(exit_action);
        combolist
    }

    pub fn display(
        mut self, window: Window, blank_callback: Option<Box<dyn FnMut(&str)>>
    ) -> Result<RustofiOptionType, ()> {
        // add enough blank lines to separate the action items
        let mut true_options = self
            .selections
            .iter()
            .map(|x| x.display.clone())
            .collect::<Vec<String>>();
        let blanks = AppRoot::calculate_blanks(
            self.selections.len() as i32,
            window.clone().dimensions.columns
        );
        true_options = true_options
            .into_iter()
            .chain(blanks.iter().map(|x| x.display.clone()).clone())
            .collect();
        true_options = true_options
            .into_iter()
            .chain(self.actions.iter().map(|x| x.display.clone()).clone())
            .collect();

        let response = window
            .clone()
            .lines(true_options.len() as i32 / window.clone().dimensions.columns)
            .show(true_options.clone());
        match response {
            Ok(input) => {
                let index;
                match usize::from_str(&input) {
                    Ok(input_num) => {
                        index = input_num;
                    }
                    Err(_) => return Ok(RustofiOptionType::Exit)
                }
                let action_start = self.selections.len() + blanks.len();

                // First handle EXIT condition
                if index == true_options.len() - 1 {
                    Ok(RustofiOptionType::Exit)
                } else if index >= action_start {
                    self.actions[(index - action_start)].run();
                    Ok(RustofiOptionType::Action)
                } else if index < self.selections.len() {
                    self.selections[index].run();
                    Ok(RustofiOptionType::Selection)
                } else if index >= self.selections.len() {
                    match blank_callback {
                        None => (AppRoot::nop)(""),
                        Some(mut cb) => (cb)("")
                    }
                    Ok(RustofiOptionType::Blank)
                } else {
                    Ok(RustofiOptionType::Exit)
                }
            }
            Err(_) => Ok(RustofiOptionType::Exit)
        }
    }
}

pub struct EntryBox {}
impl EntryBox {
    pub fn new(window: Window) -> String {
        window
            .lines(0)
            .format('s')
            .show(vec!["".to_string()])
            .expect("EntryBox")
    }
}

pub struct SelectionList {}
impl SelectionList {
    pub fn new(window: Window, options: Vec<String>) -> String {
        window
            .lines(options.len() as i32)
            .format('s')
            .show(options)
            .expect("SelectionList")
    }
}
