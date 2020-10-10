use super::schema::bindings;
use rocket_contrib::databases::diesel::prelude::*;
use rocket_contrib::databases::diesel::SqliteConnection;

#[derive(Serialize, Queryable, Identifiable, Debug)]
pub struct Binding {
    pub id: i32,
    pub default_colour: String,
    pub resting_colour: String,
    pub heaven_pin: i32,
    pub heaven_colour: String,
    pub cloud_pin: i32,
    pub cloud_colour: String,
    pub sun_pin: i32,
    pub sun_colour: String,
    pub thunder_colour: String,
    pub wind_pin: i32,
    pub wind_colour: String,
    pub water_pin: i32,
    pub water_colour: String,
    pub mountain_pin: i32,
    pub mountain_colour: String,
    pub earth_colour: String,
    pub multiply: String,
    pub bias: String,
    pub threshold: String,
    pub led_freq: i32,
    pub led_cycles: String,
    pub fan_freq: i32,
    pub fan_cycles: String,
}

#[derive(Serialize, Deserialize, Insertable, FromForm, AsChangeset, Debug)]
#[table_name = "bindings"]
pub struct UpdatedBinding {
    pub default_colour: String,
    pub resting_colour: String,
    pub heaven_pin: i32,
    pub heaven_colour: String,
    pub cloud_pin: i32,
    pub cloud_colour: String,
    pub sun_pin: i32,
    pub sun_colour: String,
    pub thunder_colour: String,
    pub wind_pin: i32,
    pub wind_colour: String,
    pub water_pin: i32,
    pub water_colour: String,
    pub mountain_pin: i32,
    pub mountain_colour: String,
    pub earth_colour: String,
    pub multiply: String,
    pub bias: String,
    pub threshold: String,
    pub led_freq: i32,
    pub led_cycles: String,
    pub fan_freq: i32,
    pub fan_cycles: String,
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
}
