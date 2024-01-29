use crate::types::ApplicationData;

const DATA_RAW: &str = include_str!("assets/data.yml");

pub fn get_data() -> Result<ApplicationData, serde_yaml::Error> {
    serde_yaml::from_str::<ApplicationData>(DATA_RAW)
}
