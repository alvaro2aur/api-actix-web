use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Info {
    pub idta: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct LevelOne {
    pub idta: i64,
    pub entity_name: String,
    pub entity_type: String,
    pub total_sector: i8,
    pub region: String,
    pub total_animals: i8,
}

#[derive(Serialize, Deserialize)]
pub struct LevelTwo {
    pub idta: i64,
    pub entity_name: String,
    pub entity_type: String,
    pub total_pavilion: u8,
    pub current_breeding: u8,
    pub sex: String,
    pub age: String,
    pub total_animals: u16,
    pub productive_state: String,
    pub cumulative_mortality: u8,
    pub average_profit: u8,
    pub conversion: u8,
    pub average_weight: f64,
    pub food: String,
    pub temperature: String,
    pub mortality: String,
    pub connectivity: String,
}

#[derive(Serialize, Deserialize)]
pub struct LevelThree {
    pub idta: i64,
    pub entity_name: String,
    pub entity_type: String,
    pub total_barnyard: u8,
    pub current_breeding: u8,
    pub sex: String,
    pub age: String,
    pub total_animals: u8,
    pub productive_state: String,
    pub cumulative_mortality: u8,
    pub average_profit: u8,
    pub conversion: u8,
    pub average_weight: f64,
    pub food: String,
    pub temperature: String,
    pub mortality: String,
    pub connectivity: String,
}

#[derive(Serialize, Deserialize)]
pub struct LevelFour {
    pub idta: i64,
    pub entity_name: String,
    pub entity_type: String,
    pub current_breeding: u8,
    pub sex: String,
    pub age: String,
    pub total_animals: u8,
    pub productive_state: String,
    pub cumulative_mortality: u8,
    pub average_profit: u8,
    pub conversion: u8,
    pub average_weight: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponseData {
    CommomLevelOne(LevelOne),
    CommomLevelTwo(LevelTwo),
    CommomLevelThree(LevelThree),
    CommomLevelFour(LevelFour),
}

#[derive(Deserialize)]
pub struct Client {
    pub client_db_schema: String,
}

#[derive(Deserialize)]
pub struct CommomBody {
    pub client: Client,
}
