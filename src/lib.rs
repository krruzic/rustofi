use crate::window::*;
use std::collections::HashMap;
use std::str;
pub mod window;

pub struct Rustofi<T, C> {
    pub response_type: ResponseType,
    pub selection: Option<T>,
    pub action: Option<C>,
}

pub enum ResponseType {
    TypeSelection,
    TypeAction,
}

impl<T: Clone, C: ToString + Clone> Rustofi<T, C> {
    pub fn new(
        prompt: &str,
        window: Window,
        selections: HashMap<String, T>,
        actions: Vec<C>,
    ) -> Result<Rustofi<T, C>, ()> {
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
                println!("INDEX IS: {:?} LENGTH IS: {:?}", data.index, selections_len);
                if data.index < selections_len {
                    // an option selected
                    Ok(Rustofi {
                        response_type: ResponseType::TypeSelection,
                        selection: selections.get(&data.entry).map(|k| k.clone()),
                        action: None,
                    })
                } else if data.index == selections_len {
                    // the empty space selected
                    Err(())
                } else if data.index > selections_len {
                    // an action selected
                    Ok(Rustofi {
                        response_type: ResponseType::TypeAction,
                        action: Some(actions[(data.index - selections_len - 1) as usize].clone()),
                        selection: None,
                    })
                } else {
                    Err(())
                }
            }
            Err(_) => Err(()),
        }
    }
}
