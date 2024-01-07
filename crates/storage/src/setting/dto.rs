use rc_entity::prelude::{SettingDataType as SettingEntityDataType, SettingModel};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SettingDataType {
    Array,
}

impl From<SettingEntityDataType> for SettingDataType {
    fn from(value: SettingEntityDataType) -> Self {
        match value {
            SettingEntityDataType::Array => SettingDataType::Array,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettingValue {
    pub key: String,
    pub raw_data: String,
    pub data_type: SettingDataType,
}

impl SettingValue {
    pub fn get_array(&self) -> Option<Vec<String>> {
        if self.data_type != SettingDataType::Array {
            tracing::warn!("setting value json formate fail");
            return None;
        }

        match serde_json::from_str::<Value>(&self.raw_data) {
            Err(_) => {
                tracing::warn!("setting value json fail");
                return None;
            }
            Ok(json_value) => {
                if let Some(array) = json_value.as_array() {
                    let mut target: Vec<String> = vec![];

                    array.iter().for_each(|object| {
                        if object.is_string() {
                            target.push(object.as_str().unwrap().to_string());
                        }
                    });

                    return Some(target);
                } else {
                    tracing::warn!("setting value json not array");

                    return None;
                }
            }
        }
    }
}

impl From<SettingModel> for SettingValue {
    fn from(value: SettingModel) -> Self {
        let SettingModel {
            key,
            raw_data,
            setting_data_type,
            ..
        } = value;

        SettingValue {
            key,
            raw_data,
            data_type: SettingDataType::from(setting_data_type),
        }
    }
}
