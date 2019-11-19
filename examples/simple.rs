use rustofi::components::ItemList;
use rustofi::RustofiResult;

fn simple_app() -> RustofiResult {
    let rustofi_entries = vec![
        "Entry 1".to_string(),
        "Entry 2".to_string(),
        "Entry 3".to_string(),
    ];
    ItemList::new(rustofi_entries, Box::new(simple_callback)).display("Select an entry".to_string())
}

pub fn simple_callback(s: &String) -> RustofiResult {
    println!("Clicked on item: {}", s);
    RustofiResult::Success
}

fn main() {
    loop {
        match simple_app() {
            RustofiResult::Error => break,
            RustofiResult::Exit => break,
            RustofiResult::Cancel => break,
            RustofiResult::Blank => break,
            _ => {}
        }
    }
}
