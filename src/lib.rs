mod errors;
pub mod window;

use crate::errors::*;
use crate::window::*;
use std::collections::HashMap;
use std::str;

pub struct Rustofi<T, C> {
    pub response_type: ResponseType,
    pub selection: Option<T>,
    pub action: Option<C>,
}

pub enum ResponseType {
    TypeSelection,
    TypeAction,
}

/// Default way of creating applications
/// A `Rustofi<T, C>` instance takes in
/// `prompt: a string displayed at the top`
/// `window: A Window object, allowing you to set the dimensions (note columns is stripped)`
/// `selections: HashMap<String, DataObject> displayed as the second section of entries in the window`
/// `actions: Vector<ActionObject> displayed as the second section of entries in the window`
/// in this example, T is a POD `DataObject` and `ActionObject` is some object that implements ToStr
///
/// The window created has the `actions` and `selections` displayed separated by a blank line,
/// if the blank line is selected an error is returned. If a `selections` item is selected, we return
/// the variant `TypeSelection` and the backing object of that selection in the HashMap.
/// If a `actions` item is selected, we return the variant `TypeAction` and the backing type of the action
impl<T: Clone, C: ToString + Clone> Rustofi<T, C> {
    pub fn new(
        prompt: &str,
        window: Window,
        selections: HashMap<String, T>,
        actions: Vec<C>,
    ) -> Result<Rustofi<T, C>, RustofiError> {
        let mut true_options: Vec<String> = selections.keys().cloned().collect();
        let selections_len = true_options.len() as i32;
        true_options.extend(vec!["".to_string()]);
        true_options = true_options
            .into_iter()
            .chain(actions.iter().map(|x| x.to_string()).clone())
            .collect();

        let response = window
            .prompt(prompt.to_string())
            .lines(true_options.len() as i32)
            .show(true_options);
        match response {
            Ok(data) => {
                if data.index < selections_len {
                    // an option selected
                    Ok(Rustofi {
                        response_type: ResponseType::TypeSelection,
                        selection: selections.get(&data.entry).map(|k| k.clone()),
                        action: None,
                    })
                } else if data.index > selections_len {
                    // an action selected
                    Ok(Rustofi {
                        response_type: ResponseType::TypeAction,
                        action: Some(actions[(data.index - selections_len - 1) as usize].clone()),
                        selection: None,
                    })
                } else {
                    // the empty space selected
                    Err(RustofiError::new(
                        RustofiErrorType::BlankLineError,
                        "The blank spacer line was selected",
                    ))
                }
            }
            Err(e) => Err(e.into()),
        }
    }
}