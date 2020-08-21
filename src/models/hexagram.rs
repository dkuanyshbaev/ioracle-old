use crate::errors::{IOracleError, IOracleResult};
use rocket_contrib::databases::rusqlite::{params, Connection};

#[derive(Serialize)]
pub struct Hexagram {
    id: i32,
    name: String,
    image: String,
    description: String,
}

#[derive(FromForm)]
pub struct UpdatedHexagram {
    pub name: String,
    pub description: String,
}

impl Hexagram {
    pub fn all(connection: &Connection) -> IOracleResult<Vec<Hexagram>> {
        let mut stmt = connection.prepare("select id, name, image, description from hexagrams")?;
        let hexagram_iter = stmt.query_map(params![], |row| {
            Ok(Hexagram {
                id: row.get(0)?,
                name: row.get(1)?,
                image: row.get(2)?,
                description: row.get(3)?,
            })
        })?;

        let mut hs: Vec<Hexagram> = Vec::new();
        for hexagram in hexagram_iter {
            if let Ok(h) = hexagram {
                hs.push(h);
            }
        }

        Ok(hs)
    }

    pub fn get(connection: &Connection, id: i32) -> IOracleResult<Hexagram> {
        let mut stmt = connection
            .prepare("select id, name, image, description from hexagrams where id = ?1")?;
        let hexagram_iter = stmt.query_map(params![id], |row| {
            Ok(Hexagram {
                id: row.get(0)?,
                name: row.get(1)?,
                image: row.get(2)?,
                description: row.get(3)?,
            })
        })?;

        let mut hs: Vec<Hexagram> = Vec::new();
        for hexagram in hexagram_iter {
            if let Ok(h) = hexagram {
                hs.push(h);
            }
        }

        match hs.pop() {
            Some(h) => Ok(h),
            None => Err(IOracleError::NotFound),
        }
    }

    pub fn update(
        connection: &Connection,
        id: i32,
        new_hexagram: UpdatedHexagram,
    ) -> IOracleResult<()> {
        connection.execute(
            "update hexagrams set name = ?1, description = ?2 where id = ?3",
            params![new_hexagram.name, new_hexagram.description, id],
        )?;

        Ok(())
    }
}
