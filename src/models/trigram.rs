use crate::errors::{IOracleError, IOracleResult};
use rocket_contrib::databases::rusqlite::{params, Connection};

#[derive(Serialize)]
pub struct Trigram {
    id: i32,
    name: String,
    image: String,
    description: String,
}

#[derive(FromForm)]
pub struct UpdatedTrigram {
    pub name: String,
    pub description: String,
}

impl Trigram {
    pub fn all(connection: &Connection) -> IOracleResult<Vec<Trigram>> {
        let mut stmt = connection.prepare("select id, name, image, description from trigrams")?;
        let trigram_iter = stmt.query_map(params![], |row| {
            Ok(Trigram {
                id: row.get(0)?,
                name: row.get(1)?,
                image: row.get(2)?,
                description: row.get(3)?,
            })
        })?;

        let mut ts: Vec<Trigram> = Vec::new();
        for trigram in trigram_iter {
            if let Ok(t) = trigram {
                ts.push(t);
            }
        }

        Ok(ts)
    }

    pub fn get(connection: &Connection, id: i32) -> IOracleResult<Trigram> {
        let mut stmt = connection
            .prepare("select id, name, image, description from trigrams where id = ?1")?;
        let trigram_iter = stmt.query_map(params![id], |row| {
            Ok(Trigram {
                id: row.get(0)?,
                name: row.get(1)?,
                image: row.get(2)?,
                description: row.get(3)?,
            })
        })?;

        let mut ts: Vec<Trigram> = Vec::new();
        for trigram in trigram_iter {
            if let Ok(t) = trigram {
                ts.push(t);
            }
        }

        match ts.pop() {
            Some(t) => Ok(t),
            None => Err(IOracleError::NotFound),
        }
    }

    pub fn update(connection: &Connection, id: i32, trigram: UpdatedTrigram) -> IOracleResult<()> {
        connection.execute(
            "update trigrams set name = ?1, description = ?2 where id = ?3",
            params![trigram.name, trigram.description, id],
        )?;

        Ok(())
    }
}
