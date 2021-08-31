use rusqlite::{Connection, Result};

mod task;

use task::Task;

// TODO Add today date
// TODO Add UI

fn main() -> Result<()> {
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

    let task = Task::new(
        "Test1".to_string(),
        "2222-01-01".to_string(),
        "2222-01-01".to_string(),
    );

    let tasks = task.get(&conn)?;

    for t in tasks {
        println!("{:#?}", t);
    }

    Ok(())
}
