use fltk::{app, prelude::*, window::Window};
use rusqlite::{Connection, Result};

mod task;
use task::Task;

// TODO Add UI

fn main() -> Result<()> {
    let app = app::App::default();
    let mut win = Window::new(100, 100, 400, 300, "Task Tracker");
    let conn = Connection::open("task_rust.db")?;

    conn.execute(
        "create table if not exists Task (
            id          INTEGER PRIMARY KEY,
            desc        TEXT NOT NULL,
            due_date    DATE,
            created_at  DATE
        )",
        [],
    )?;

    let task = Task::new("Test1".to_string(), "2222-01-01".to_string());

    task.insert(&conn)?;

    let tasks = task.get(&conn)?;
    for t in tasks {
        println!("{:#?}", t);
    }

    win.end();
    win.show();
    app.run().unwrap();

    Ok(())
}
