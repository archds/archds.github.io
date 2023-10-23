use crate::types::ApplicationData;

const DATA_RAW: &str = include_str!("data.json");

pub fn get_data() -> Result<ApplicationData, serde_json::Error> {
    serde_json::from_str::<ApplicationData>(DATA_RAW)
}
