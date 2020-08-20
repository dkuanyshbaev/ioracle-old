use crate::errors::IOracleResult;
use rocket_contrib::databases::rusqlite::{params, Connection};

pub fn init(connection: &Connection) -> IOracleResult<()> {
    connection.execute(
        "create table if not exists answers (
            id integer primary key,
            uuid text not null,
            email text not null,
            question text not null,
            answer text not null,
            created_at datetime default current_timestamp
        )",
        params![],
    )?;

    connection.execute(
        "create table if not exists trigrams (
            id integer primary key,
            name text not null,
            image text not null,
            description text not null
        )",
        params![],
    )?;

    connection.execute(
        "create table if not exists hexagrams (
            id integer primary key,
            name text not null,
            image text not null,
            description text not null
        )",
        params![],
    )?;

    // we have 8 basic trigrams
    for i in 1..=8 {
        connection.execute(
            "insert or ignore into trigrams (id, name, image, description) values (?1, ?2, ?3, ?4)",
            params![
                i,
                "".to_string(),
                format!("{}.png", i.to_string()),
                "".to_string()
            ],
        )?;
    }

    // and 64 hexagrams
    for i in 1..=64 {
        connection.execute(
            "insert or ignore into hexagrams (id, name, image, description) values (?1, ?2, ?3, ?4)",
            params![
                i,
                "".to_string(),
                format!("{}.png", i.to_string()),
                "".to_string()
            ],
        )?;
    }

    Ok(())
}
