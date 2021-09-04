use rusqlite::{Connection, Result};
use std::env::args;
use std::io::*;

mod task;
use task::Task;

// TODO Add UI

fn read_input() -> Task {
    let mut desc: String = String::new();
    println!("Enter Task");
    stdin()
        .read_line(&mut desc)
        .expect("Error reading description");

    let mut due_date: String = String::new();
    println!("Enter Due Date");
    stdin()
        .read_line(&mut due_date)
        .expect("Error reading due date");

    Task::new(desc, due_date)
}

fn main() -> Result<()> {
    let conn = Connection::open("task_rust.db")?;
    let arg: String = args().nth(1).unwrap_or(String::new());

    conn.execute(
        "create table if not exists Task (
            id          INTEGER PRIMARY KEY,
            desc        TEXT NOT NULL,
            due_date    DATE,
            created_at  DATE
        )",
        [],
    )?;

    match arg.as_str() {
        "add" => {
            let task = read_input();
            task.insert(&conn)?;
            println!("Task {} added successfully", task.id);
        }
        "get" => {
            let tasks = Task::get(&conn)?;
            let tasks_str: Vec<String> = Task::convert(tasks)?;
            println!("{:#?}", tasks_str);
        }
        "help" => println!("Help"),
        _ => println!("Undefined argument!"),
    }

    Ok(())
}
