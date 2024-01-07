use rc_entity::prelude::{SettingDataType as SettingEntityDataType, SettingModel};

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct SettingValue {
    key: String,
    raw_data: String,
    data_type: SettingDataType,
}

impl From<SettingModel> for SettingValue {
    fn from(value: SettingModel) -> Self {
        let SettingModel {
            key,
            raw_data,
            setting_data_type,
            create_at,
            update_at,
            ..
        } = value;

        SettingValue {
            key,
            raw_data,
            data_type: SettingDataType::from(setting_data_type),
        }
    }
}
