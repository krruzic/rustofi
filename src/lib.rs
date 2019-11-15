mod errors;
pub mod window;

use crate::window::*;

use std::str;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum RustofiOptionType {
    /// default entry
    Selection,
    /// blank line to separate `Selection` entries and `Action` entries
    Blank,
    /// action entry, separated from defaults with Blank
    Action,
    /// action designed to exit the application
    Exit
}

/// A `RustofiOption` represents an entry in rofi's list
pub struct RustofiOption {
    /// what rofi should display for this option
    pub display: String,
    /// function to run when rofi returns this entry, along with its string representation
    pub callback: Box<dyn FnMut(&str)>,
    /// what type of entry is it
    pub option: RustofiOptionType
}

impl RustofiOption {
    fn run(&mut self) {
        (self.callback)(&self.display);
    }
}

/// A rofi window that represents a root control in a GUI Application
/// it contains two main fields:
/// `selections`: simple text entries to display
/// `actions`: entries designed to run an operation without any input
///
/// Typically you will want to create an AppRoot with some options and actions,
/// then display it in a loop checking `response` for the `RustofiOptionType` to exit on
/// An AppRoot will automatically add an [Exit] option to simplify the loop exit cases
///
/// ```no_run
/// use rustofi::{AppRoot, RustofiOption, RustofiOptionType};
/// use rustofi::window::{Window, Location};
///
/// fn simple_app() -> Result<RustofiOptionType, ()>{
///     let mut rustofi_entries: Vec<RustofiOption> = Vec::new();
///     let entries = vec!["Entry 1", "Entry 2", "Entry 3"];
///     for entry in entries {
///         rustofi_entries.push(RustofiOption {
///             display: entry.to_string(),
///             callback: Box::new(simple_callback),
///             option: RustofiOptionType::Selection
///         });
///     }
///     AppRoot::new(rustofi_entries).display(create_window(), None)
/// }
///
/// fn simple_callback(s: &str) {
///     println!("Clicked on {}", s.to_string());
/// }
///
/// fn create_window() -> Window<'static> {
///     Window::new("A Simple Rustofi App")
///         .format('i')
///         .location(Location::MiddleCentre)
///         .add_args(vec!["-markup-rows".to_string()])
/// }
///
/// fn main() {
///     loop {
///         match simple_app() {
///             Ok(m) => {
///                 println!("Returned: {:?}", m);
///                 match m {
///                     RustofiOptionType::Exit => break,
///                     _ => {}
///                 }
///             }
///             _ => break
///         }
///     }
/// }
/// ```
pub struct AppRoot {
    /// text entries to display, use these for options that will run simple processes
    pub selections: Vec<RustofiOption>,
    /// action entries to display, use these for options that will launch a `SelectionList`
    pub actions: Vec<RustofiOption>,
    /// which type of entry was selected
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

    /// create an AppRoot with given entries
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

    /// run the rofi command and parse its return value
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

/// Display a window with no entries and no rows
/// used for returning a user entry
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

/// Display a window with actions and a row of valid selections
/// used for returning a user selection
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
