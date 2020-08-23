use super::schema::bindings;
use crate::errors::{IOracleError, IOracleResult};
use rocket_contrib::databases::diesel::prelude::*;
use rocket_contrib::databases::diesel::SqliteConnection;
use serde_json::to_writer;
use serde_json::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

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
    multiply: String,
    bias: String,
    threshold: String,
}

#[derive(Serialize, Deserialize, Insertable, FromForm, AsChangeset, Debug)]
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
    multiply: String,
    bias: String,
    threshold: String,
}

impl Binding {
    pub fn get(connection: &SqliteConnection) -> QueryResult<Binding> {
        bindings::table.find(1).get_result(connection)
    }

    pub fn update(connection: &SqliteConnection, bindings: UpdatedBinding) -> QueryResult<usize> {
        diesel::update(bindings::table.find(1))
            .set(bindings)
            .execute(connection)
    }

    pub fn write_to_file(connection: &SqliteConnection) -> IOracleResult<()> {
        let current_bindings = Self::get(connection)?;
        let path = Path::new(&current_bindings.file_name);

        match File::create(&path) {
            Err(err) => Err(IOracleError::InternalServerError),
            Ok(file) => match to_writer(file, &current_bindings) {
                Err(err) => Err(IOracleError::InternalServerError),
                Ok(_) => Ok(()),
            },
        }
    }

    pub fn read_from_file(file_name: String) -> IOracleResult<()> {
        println!("{}", file_name);
        Ok(())
    }
}
