// examples/simple.rs
use rustofi::components::ItemList;
use rustofi::RustofiResult;

fn simple_app() -> RustofiResult {
    // create a list of strings to pass as rofi options. Note that this can be any type you want,
    // the callback will always return the type you passed in
    let rustofi_entries = vec![
        "Entry 1".to_string(),
        "Entry 2".to_string(),
        "Entry 3".to_string(),
    ];
    // create a ItemList with a callback that prints which item was selected.
    ItemList::new(rustofi_entries, Box::new(simple_callback)).display("Select an entry".to_string())
}

pub fn simple_callback(s: &String) -> RustofiResult {
    // when an item is clicked, print the name!
    println!("Clicked on item: {}", s);
    RustofiResult::Success
}

fn main() {
    loop {
        match simple_app() {
            // loop unless the user requests we exit
            RustofiResult::Error(_) => break,
            RustofiResult::Exit => break,
            RustofiResult::Cancel => break,
            RustofiResult::Blank => break,
            _ => {}
        }
    }
}
