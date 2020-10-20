use super::schema::hexagrams;
use rocket_contrib::databases::diesel::prelude::*;
use rocket_contrib::databases::diesel::SqliteConnection;

#[derive(Serialize, Queryable, Identifiable, Debug)]
pub struct Hexagram {
    pub id: i32,
    pub binary: String,
    pub king_wen_order: i32,
    pub shao_yong_order: i32,
    pub gua: String,
    pub pin_yin: String,
    pub character: String,
    pub wilheim: String,
    pub huang: String,
    pub hatcher: String,
    pub no2do: String,
    pub inner_ba_gua: String, // first_trigram
    pub outer_ba_gua: String, // second_trigram
    pub host_yao: String,
    pub judgment: String,
    pub image: String,
    pub lines: String,
}

#[derive(Serialize, Deserialize, Insertable, FromForm, AsChangeset, Debug)]
#[table_name = "hexagrams"]
#[serde(rename_all = "PascalCase")]
pub struct UpdatedHexagram {
    pub binary: String,
    pub king_wen_order: i32,
    pub shao_yong_order: i32,
    pub gua: String,
    pub pin_yin: String,
    pub character: String,
    pub wilheim: String,
    pub huang: String,
    pub hatcher: String,
    pub no2do: String,
    pub inner_ba_gua: String,
    pub outer_ba_gua: String,
    pub host_yao: String,
    pub judgment: String,
    pub image: String,
    pub lines: String,
}

#[derive(Serialize, Deserialize, Insertable, FromForm, AsChangeset, Debug)]
#[table_name = "hexagrams"]
#[serde(rename_all = "PascalCase")]
pub struct SheetsHexagram {
    pub binary: String,
    pub king_wen_order: i32,
    pub shao_yong_order: i32,
    pub gua: String,
    pub pin_yin: String,
    pub character: String,
    pub wilheim: String,
    pub huang: String,
    pub hatcher: String,
    pub no2do: String,
    pub inner_ba_gua: String,
    pub outer_ba_gua: String,
    pub host_yao: String,
}

impl Hexagram {
    pub fn all(connection: &SqliteConnection) -> QueryResult<Vec<Hexagram>> {
        hexagrams::table.order(hexagrams::id.asc()).load(connection)
    }

    pub fn get(connection: &SqliteConnection, id: i32) -> QueryResult<Hexagram> {
        hexagrams::table.find(id).get_result(connection)
    }

    pub fn get_by_binary(connection: &SqliteConnection, binary: String) -> QueryResult<Hexagram> {
        hexagrams::table
            .filter(hexagrams::binary.eq(&binary))
            .first(connection)
    }

    pub fn insert(
        connection: &SqliteConnection,
        new_hexagram: UpdatedHexagram,
    ) -> QueryResult<usize> {
        diesel::insert_into(hexagrams::table)
            .values(new_hexagram)
            .execute(connection)
    }

    pub fn update(
        connection: &SqliteConnection,
        id: i32,
        new_hexagram: UpdatedHexagram,
    ) -> QueryResult<usize> {
        diesel::update(hexagrams::table.find(id))
            .set(new_hexagram)
            .execute(connection)
    }
}
