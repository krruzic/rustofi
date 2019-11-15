#[macro_use]
extern crate lazy_static;

use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use rustofi::window::{Dimensions, Location, Window};
use rustofi::{AppRoot, EntryBox, RustofiOption, RustofiOptionType, SelectionList};
use serde::{Deserialize, Serialize};

use std::fmt::{self};

use std::string::ToString;
use std::sync::Mutex;

fn get_db() -> PickleDb {
    let mut db;
    match PickleDb::load(
        "todo.db",
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Json
    ) {
        Ok(pickledb) => db = pickledb,
        Err(_) => {
            db = PickleDb::new(
                "todo.db",
                PickleDbDumpPolicy::AutoDump,
                SerializationMethod::Json
            )
        }
    };
    if let false = db.lexists("TodoList") {
        db.lcreate("TodoList").expect("Failed to create DB");
    }
    db
}

lazy_static! {
    static ref DB: Mutex<PickleDb> = { Mutex::new(get_db()) };
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TodoStatus {
    Todo,
    Complete
}
impl fmt::Display for TodoStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TodoStatus::Todo => write!(f, "TODO"),
            TodoStatus::Complete => write!(f, "COMPLETE")
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodoItem {
    pub status: TodoStatus,
    pub task: String
}
impl fmt::Display for TodoItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.status {
            TodoStatus::Todo => write!(f, "{}", self.task),
            TodoStatus::Complete => write!(f, "<s>{}</s>", self.task)
        }
    }
}

pub struct TodoApp {}

fn create_window() -> Window<'static> {
    Window::new("Today's Todo list")
        .format('i')
        .location(Location::MiddleCentre)
        .message("Select an item to mark it as complete, select the blank row to add a new item")
        .dimensions(Dimensions {
            width: 720,
            height: 640,
            lines: 0,
            columns: 1
        })
        .add_args(vec!["-markup-rows".to_string()])
}

impl TodoApp {
    pub fn add_item(task: String) {
        if task == "" {
            return;
        }
        DB.lock().unwrap().ladd(
            "TodoList",
            &TodoItem {
                status: TodoStatus::Todo,
                task
            }
        );
    }

    pub fn delete_item(s: &str) {
        println!("Deleting task: {}", s);
        // remove strikethrough
        let stripped_s = s.to_string().replace("<s>", "").replace("</s>", "");
        // have to check if item exists both completed and uncompleted
        DB.lock()
            .unwrap()
            .lrem_value(
                "TodoList",
                &TodoItem {
                    status: TodoStatus::Todo,
                    task: stripped_s.clone()
                }
            )
            .expect("Failed to remove entry");
        DB.lock()
            .unwrap()
            .lrem_value(
                "TodoList",
                &TodoItem {
                    status: TodoStatus::Complete,
                    task: stripped_s
                }
            )
            .expect("Failed to remove entry");
    }

    pub fn mark_done(s: &str) {
        println!("Marking task: {} as done", s);

        // have to remove item then readd it
        DB.lock()
            .unwrap()
            .lrem_value(
                "TodoList",
                &TodoItem {
                    status: TodoStatus::Todo,
                    task: s.to_string()
                }
            )
            .expect("Failed to remove entry");
        DB.lock()
            .unwrap()
            .ladd(
                "TodoList",
                &TodoItem {
                    status: TodoStatus::Complete,
                    task: s.to_string()
                }
            )
            .expect("Failed to remove entry");
    }

    pub fn show_todos() -> Result<RustofiOptionType, ()> {
        // add all TodoItems to the list of rofi selections
        let mut todo_list: Vec<RustofiOption> = Vec::new();
        for item_iter in DB.lock().unwrap().liter("TodoList") {
            let item = item_iter.get_item::<TodoItem>().unwrap();
            println!("{}", item.to_string());
            todo_list.push(RustofiOption {
                display: item.to_string(),
                callback: Box::new(TodoApp::mark_done),
                option: RustofiOptionType::Selection
            });
        }
        todo_list.push(RustofiOption {
            display: "[Delete]".to_string(),
            callback: Box::new(TodoApp::delete_prompt),
            option: RustofiOptionType::Action
        });
        AppRoot::new(todo_list).display(create_window(), Some(Box::new(TodoApp::create_prompt)))
    }

    pub fn delete_prompt(_s: &str) {
        let mut todo_list: Vec<String> = Vec::new();
        for item_iter in DB.lock().unwrap().liter("TodoList") {
            let item = item_iter.get_item::<TodoItem>().unwrap();
            println!("{}", item.to_string());
            todo_list.push(item.to_string());
        }
        let key = SelectionList::new(
            create_window().prompt("Select the Todo to delete".to_string()),
            todo_list
        );
        TodoApp::delete_item(&key);
    }

    pub fn create_prompt(_s: &str) {
        let m = EntryBox::new(create_window().prompt("Enter a new TODO".to_string()));
        TodoApp::add_item(m);
    }

    pub fn drop_list() {
        DB.lock()
            .unwrap()
            .lrem_list("TodoList")
            .expect("Failed to remove list");
    }
}

/// cargo run --example `todo_app`
/// When the rofi menu appears, click the blank entry to create a new todo_list item
fn main() {
    loop {
        match TodoApp::show_todos() {
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
