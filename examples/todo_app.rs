#[macro_use]
extern crate lazy_static;

use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use rustofi::components::ActionList;
use rustofi::components::EntryBox;
use rustofi::components::ItemList;
use rustofi::window::{Dimensions, Location, Window};
use rustofi::AppPage;
use rustofi::CallbackResult;
use rustofi::RustofiComponent;
use rustofi::RustofiResult;
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
impl TodoItem {
    pub fn toggle(&mut self) {
        match self.status {
            TodoStatus::Todo => self.status = TodoStatus::Complete,
            TodoStatus::Complete => self.status = TodoStatus::Todo
        }
    }
}

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

pub enum TodoState {
    Root,
    Add,
    Delete,
    Exit
}

pub enum TodoAction {
    Add,
    Delete
}

pub struct TodoApp {}

impl TodoApp {
    pub fn add(t: &TodoItem) {
        DB.lock().unwrap().ladd("TodoList", &t);
    }
    pub fn delete(t: &mut TodoItem) -> CallbackResult {
        DB.lock().unwrap().lrem_value("TodoList", &t);
        Ok(())
    }
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

    pub fn toggle_todo(t: &mut TodoItem) -> CallbackResult {
        println!("Marking task: {}", t);

        // have to remove item then readd it
        match DB.lock().unwrap().lrem_value("TodoList", &t) {
            Ok(_) => {}
            Err(_) => return Err("error toggling todo".to_string())
        };
        t.toggle();
        TodoApp::add(t);
        Ok(())
    }

    pub fn show_todos() -> RustofiResult {
        // add all TodoItems to the list of rofi selections
        let mut todos = Vec::new();
        for item_iter in DB.lock().unwrap().liter("TodoList") {
            let item = item_iter.get_item::<TodoItem>().unwrap();
            todos.push(item);
        }
        AppPage::<TodoItem>::new(
            todos,
            Box::new(TodoApp::toggle_todo),
            vec!["[add]".to_string(), "[delete]".to_string()]
        )
        .window(create_window())
        .display("Todo".to_string())
    }
    pub fn delete_todos() -> RustofiResult {
        let mut todos = Vec::new();
        for item_iter in DB.lock().unwrap().liter("TodoList") {
            let item = item_iter.get_item::<TodoItem>().unwrap();
            todos.push(item);
        }
        ItemList::<TodoItem>::new(todos, Box::new(TodoApp::delete))
            .display("Select a Todo to delete".to_string())
    }

    pub fn add_todo() -> RustofiResult {
        EntryBox::display("Enter a new Todo".to_string())
    }
}

/// cargo run --example `todo_app`
/// When the rofi menu appears, click the blank entry to create a new todo_list item
fn main() {
    let mut state = TodoState::Root;
    loop {
        state = match state {
            // handle input in the root state that lists
            // all todos and [delete], [add], [exit]
            TodoState::Root => match TodoApp::show_todos() {
                RustofiResult::Action(a) => {
                    if a == "[add]".to_string() {
                        // switch to the add todo page next 'frame'
                        TodoState::Add
                    } else if a == "[delete]".to_string() {
                        TodoState::Delete
                    } else {
                        // exit next frame
                        TodoState::Exit
                    }
                }
                // marked as done, continue displaying list
                RustofiResult::Selection(s) => TodoState::Root,
                _ => TodoState::Exit // something weird happened, just exit
            },
            // handle input in the add state
            TodoState::Add => match TodoApp::add_todo() {
                RustofiResult::Selection(s) => {
                    TodoApp::add_item(s);
                    TodoState::Root // todo added, return to main page
                }
                // todo cancelled, return to main page
                RustofiResult::Cancel => TodoState::Root,
                _ => break // something weird happened, crash down!
            },
            // handle input in the delete state
            TodoState::Delete => match TodoApp::delete_todos() {
                // deleted
                RustofiResult::Selection(s) => TodoState::Root,
                // cancelled
                RustofiResult::Cancel => TodoState::Root,
                _ => break
            },
            TodoState::Exit => break,
            _ => break
        };
    }
}
