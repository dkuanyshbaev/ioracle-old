use super::schema::hexagrams;
use rocket_contrib::databases::diesel::prelude::*;
use rocket_contrib::databases::diesel::SqliteConnection;

#[derive(Serialize, Queryable, Identifiable, Debug)]
pub struct Hexagram {
    id: i32,
    name: String,
    image: String,
    description: String,
}

#[derive(Serialize, Insertable, FromForm, AsChangeset)]
#[table_name = "hexagrams"]
pub struct UpdatedHexagram {
    pub name: String,
    pub description: String,
}

impl Hexagram {
    pub fn all(connection: &SqliteConnection) -> QueryResult<Vec<Hexagram>> {
        hexagrams::table.order(hexagrams::id.asc()).load(connection)
    }

    pub fn get(connection: &SqliteConnection, id: i32) -> QueryResult<Hexagram> {
        hexagrams::table.find(id).get_result(connection)
    }

    pub fn update(
        connection: &SqliteConnection,
        id: i32,
        new_hexagram: UpdatedHexagram,
    ) -> QueryResult<usize> {
        // let old_hexagram = Self::get(connection, id)?;
        // diesel::update(&old_hexagram)
        //     .set(new_hexagram)
        //     .execute(connection)
        diesel::update(hexagrams::table.find(id))
            .set(new_hexagram)
            .execute(connection)
    }
}
