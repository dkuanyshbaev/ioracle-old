use super::schema::trigrams;
use rocket_contrib::databases::diesel::prelude::*;
use rocket_contrib::databases::diesel::SqliteConnection;

#[derive(Serialize, Queryable, Identifiable, Debug)]
pub struct Trigram {
    id: i32,
    name: String,
    image: String,
    binary: String,
    no: String,
    wen: String,
    host: String,
    element: String,
}

#[derive(Serialize, Deserialize, Insertable, FromForm, AsChangeset, Debug)]
#[table_name = "trigrams"]
#[serde(rename_all = "PascalCase")]
pub struct UpdatedTrigram {
    name: String,
    image: String,
    binary: String,
    no: String,
    wen: String,
    host: String,
    element: String,
}

impl Trigram {
    pub fn all(connection: &SqliteConnection) -> QueryResult<Vec<Trigram>> {
        trigrams::table.order(trigrams::id.asc()).load(connection)
    }

    pub fn get(connection: &SqliteConnection, id: i32) -> QueryResult<Trigram> {
        trigrams::table.find(id).get_result(connection)
    }

    pub fn insert(
        connection: &SqliteConnection,
        new_trigram: UpdatedTrigram,
    ) -> QueryResult<usize> {
        diesel::insert_into(trigrams::table)
            .values(new_trigram)
            .execute(connection)
    }

    pub fn update(
        connection: &SqliteConnection,
        id: i32,
        new_trigram: UpdatedTrigram,
    ) -> QueryResult<usize> {
        diesel::update(trigrams::table.find(id))
            .set(new_trigram)
            .execute(connection)
    }
}
