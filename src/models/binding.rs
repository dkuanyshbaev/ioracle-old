use super::schema::bindings;
use crate::errors::IOracleResult;
use rocket_contrib::databases::diesel::prelude::*;
use rocket_contrib::databases::diesel::SqliteConnection;

#[derive(Serialize, Queryable, Identifiable, Debug)]
pub struct Binding {
    id: i32,
    file_name: String,
    heaven_pin: i32,
    heaven_colour: String,
    cloud_pin: i32,
    cloud_colour: String,
    sun_pin: i32,
    sun_colour: String,
    wind_pin: i32,
    wind_colour: String,
    thunder_sound: String,
    thunder_colour: String,
    water_pin: i32,
    water_colour: String,
    mountain_sound: String,
    mountain_colour: String,
    earth_pin: i32,
    earth_colour: String,
    multiply: f32,
    bias: f32,
    threshold: f32,
}

#[derive(Serialize, Deserialize, Insertable, FromForm, AsChangeset)]
#[table_name = "bindings"]
pub struct UpdatedBinding {
    file_name: String,
    heaven_pin: i32,
    heaven_colour: String,
    cloud_pin: i32,
    cloud_colour: String,
    sun_pin: i32,
    sun_colour: String,
    wind_pin: i32,
    wind_colour: String,
    thunder_sound: String,
    thunder_colour: String,
    water_pin: i32,
    water_colour: String,
    mountain_sound: String,
    mountain_colour: String,
    earth_pin: i32,
    earth_colour: String,
    multiply: f32,
    bias: f32,
    threshold: f32,
}

impl Binding {
    pub fn get(connection: &SqliteConnection, id: i32) -> QueryResult<Binding> {
        bindings::table.find(id).get_result(connection)
    }

    pub fn update(connection: &SqliteConnection, bindings: UpdatedBinding) -> QueryResult<usize> {
        diesel::update(bindings::table.find(1))
            .set(bindings)
            .execute(connection)
    }

    //     pub fn apply(&self, connection: &SqliteConnection) -> IOracleResult<()> {
    //         // connection.execute(
    //         //     "update settings set name = ?1, description = ?2 where id = ?3",
    //         //     params![trigram.name, trigram.description, id],
    //         // )?;
    //
    //         Ok(())
    //     }
    //
    //     pub fn write(&self) -> IOracleResult<()> {
    //         // to file
    //         Ok(())
    //     }
    //
    //     pub fn read(&self) -> IOracleResult<()> {
    //         // from file
    //         Ok(())
    //     }
}
