use rustofi::{AppRoot, RustofiOption, RustofiOptionType};
use rustofi::window::{Window, Location};

fn simple_app() -> Result<RustofiOptionType, ()>{
    let mut rustofi_entries: Vec<RustofiOption> = Vec::new();
    let entries = vec!["Entry 1", "Entry 2", "Entry 3"];
    for entry in entries {
        rustofi_entries.push(RustofiOption {
            display: entry.to_string(),
            callback: Box::new(simple_callback),
            option: RustofiOptionType::Selection
        });
    }
    AppRoot::new(rustofi_entries).display(create_window(), None)
}

fn simple_callback(s: &str) {
    println!("Clicked on {}", s.to_string());
}

fn create_window() -> Window<'static> {
    Window::new("A Simple Rustofi App")
        .format('i')
        .location(Location::MiddleCentre)
        .add_args(vec!["-markup-rows".to_string()])
}

/// Bare minimum application demonstrating the use of Rustofi::AppRoot
fn main() {
    loop {
        match simple_app() {
            Ok(m) => {
                println!("Returned: {:?}", m);
                match m {
                    RustofiOptionType::Exit => break,
                    _ => {}
                }
            }
            _ => break
        }
    }
}
