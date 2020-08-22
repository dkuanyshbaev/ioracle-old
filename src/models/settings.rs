use super::schema::settings;
use crate::errors::IOracleResult;
use rocket_contrib::databases::diesel::prelude::*;
use rocket_contrib::databases::diesel::SqliteConnection;

// #[derive(Serialize, Queryable, Identifiable)]
#[derive(Serialize, Queryable)]
pub struct Settings {
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

#[derive(Serialize, Deserialize, Insertable, AsChangeset, Debug)]
#[table_name = "settings"]
pub struct UpdatedSettings {
    pub file_name: String,
    pub heaven_pin: i32,
    pub heaven_colour: String,
    pub cloud_pin: i32,
    pub cloud_colour: String,
    pub sun_pin: i32,
    pub sun_colour: String,
    pub wind_pin: i32,
    pub wind_colour: String,
    pub thunder_sound: String,
    pub thunder_colour: String,
    pub water_pin: i32,
    pub water_colour: String,
    pub mountain_sound: String,
    pub mountain_colour: String,
    pub earth_pin: i32,
    pub earth_colour: String,
    pub multiply: f32,
    pub bias: f32,
    pub threshold: f32,
}

impl Settings {
    pub fn get(connection: &SqliteConnection, id: i32) -> QueryResult<Settings> {
        settings::table.find(id).get_result(connection)
    }

    pub fn update(
        connection: &SqliteConnection,
        updated_settings: UpdatedSettings,
    ) -> QueryResult<usize> {
        let old_settings = Self::get(connection, 1)?;
        diesel::update(&old_settings)
            .set(updated_settings)
            .execute(connection)
    }

    pub fn apply(&self, connection: &SqliteConnection) -> IOracleResult<()> {
        // connection.execute(
        //     "update settings set name = ?1, description = ?2 where id = ?3",
        //     params![trigram.name, trigram.description, id],
        // )?;

        Ok(())
    }

    pub fn write(&self) -> IOracleResult<()> {
        // to file
        Ok(())
    }

    pub fn read(&self) -> IOracleResult<()> {
        // from file
        Ok(())
    }
}
