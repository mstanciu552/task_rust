use rusqlite::{params, Connection, Result};

mod task;

use task::Task;

// TODO Add today date

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

    let task = Task {
        id: Default::default(),
        desc: "Test1".to_string(),
        due_date: "2021-08-30".to_string(),
        created_at: "2021-08-30".to_string(),
    };

    conn.execute(
        "insert into Task (desc, due_date, created_at) values (?1, ?2, ?3)",
        params![task.desc, task.due_date, task.created_at],
    )?;

    let mut stmt = conn.prepare("select * from Task")?;
    let tasks = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            desc: row.get(1)?,
            due_date: row.get(2)?,
            created_at: row.get(3)?,
        })
    })?;

    for t in tasks {
        println!("{:#?}", t.unwrap());
    }

    Ok(())
}
