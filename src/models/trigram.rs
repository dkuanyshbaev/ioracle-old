use super::schema::trigrams;
use rocket_contrib::databases::diesel::prelude::*;
use rocket_contrib::databases::diesel::SqliteConnection;

#[derive(Serialize, Queryable, Identifiable, Debug)]
pub struct Trigram {
    id: i32,
    name: String,
    image: String,
    description: String,
}

#[derive(Serialize, Insertable, FromForm, AsChangeset)]
#[table_name = "trigrams"]
pub struct UpdatedTrigram {
    pub name: String,
    pub description: String,
}

impl Trigram {
    pub fn all(connection: &SqliteConnection) -> QueryResult<Vec<Trigram>> {
        trigrams::table.order(trigrams::id.asc()).load(connection)
    }

    pub fn get(connection: &SqliteConnection, id: i32) -> QueryResult<Trigram> {
        trigrams::table.find(id).get_result(connection)
    }

    pub fn update(
        connection: &SqliteConnection,
        id: i32,
        new_trigram: UpdatedTrigram,
    ) -> QueryResult<usize> {
        let old_trigram = Self::get(connection, id)?;
        diesel::update(&old_trigram)
            .set(new_trigram)
            .execute(connection)
    }
}
