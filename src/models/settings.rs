use crate::errors::IOracleResult;
use rocket_contrib::databases::diesel::SqliteConnection;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub file_name: String,
    pub heaven_pin: u8,
    pub heaven_colour: String,
    pub cloud_pin: u8,
    pub cloud_colour: String,
    pub sun_pin: u8,
    pub sun_colour: String,
    pub wind_pin: u8,
    pub wind_colour: String,
    pub thunder_sound: String,
    pub thunder_colour: String,
    pub water_pin: u8,
    pub water_colour: String,
    pub mountain_sound: String,
    pub mountain_colour: String,
    pub earth_pin: u8,
    pub earth_colour: String,
    pub multiply: f32,
    pub bias: f32,
    pub threshold: f32,
}

impl Settings {
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
