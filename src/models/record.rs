use super::schema::records;
use rocket_contrib::databases::diesel::prelude::*;
use rocket_contrib::databases::diesel::SqliteConnection;

#[derive(Serialize, Queryable, Identifiable, Debug)]
pub struct Record {
    pub id: i32,
    pub uuid: String,
    pub email: String,
    pub question: String,
    pub answer: String,
    pub hexagram: String,
}

#[derive(Serialize, Deserialize, Insertable, FromForm, AsChangeset, Debug)]
#[table_name = "records"]
pub struct UpdatedRecord {
    pub uuid: String,
    pub email: String,
    pub question: String,
    pub answer: String,
    pub hexagram: String,
}

impl Record {
    pub fn get(connection: &SqliteConnection, id: i32) -> QueryResult<Record> {
        records::table.find(id).get_result(connection)
    }

    pub fn get_by_uuid(connection: &SqliteConnection, uuid: String) -> QueryResult<Record> {
        records::table
            .filter(records::uuid.eq(&uuid))
            .first(connection)
    }

    pub fn insert(connection: &SqliteConnection, new_record: UpdatedRecord) -> QueryResult<usize> {
        diesel::insert_into(records::table)
            .values(new_record)
            .execute(connection)
    }
}
