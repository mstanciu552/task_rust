use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub struct Task {
    pub id: i32,
    pub desc: String,
    pub due_date: String,
    pub created_at: String,
}

impl Task {
    pub fn new(desc: String, due_date: String, created_at: String) -> Self {
        Self {
            id: Default::default(),
            desc,
            due_date,
            created_at,
        }
    }

    pub fn insert(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "insert into Task (desc, due_date, created_at) values (?1, ?2, ?3)",
            params![self.desc, self.due_date, self.created_at],
        )?;
        Ok(())
    }

    pub fn get(&self, conn: &Connection) -> Result<Vec<Self>> {
        let mut stmt = conn.prepare("select * from Task")?;
        let res = stmt.query_map([], |row| {
            Ok(Self {
                id: row.get(0)?,
                desc: row.get(1)?,
                due_date: row.get(2)?,
                created_at: row.get(3)?,
            })
        })?;

        let mut tasks: Vec<Self> = Vec::new();

        for task in res {
            tasks.push(task.unwrap());
        }

        Ok(tasks)
    }

    pub fn delete(conn: &Connection, id: i32) -> Result<()> {
        conn.execute("delete from Task where id=?1", params![id])?;
        Ok(())
    }

    pub fn update(conn: &Connection, id: i32, task: Self) -> Result<()> {
        conn.execute(
            "update Task set desc=?1, due_date=?2, created_at=?3 where id=?4",
            params![task.desc, task.due_date, task.created_at, id],
        )?;
        Ok(())
    }
}
