use crate::{encryptor::Encryptor, state::State, ServerKind, ServerResult};

use rand::{thread_rng, Rng};
use rc_entity::sea_orm::TransactionTrait;
use rc_storage::prelude::{User, UserForm, UserFormEncrypt, UserStorage};

pub struct UserService<'a> {
    state: &'a State,
}

impl<'a> UserService<'a> {
    pub fn new(state: &'a State) -> Self {
        UserService { state }
    }

    pub async fn get_range_avatar(&self) -> ServerResult<String> {
        let guard = self.state.setting.read().await;
        let setting_value = (*guard)
            .0
            .get("avatar")
            .and_then(|value| Some(value.clone()));

        if let Some(setting_value) = setting_value {
            if let Some(images) = setting_value.get_array() {
                let mut rng = thread_rng();

                let index = rng.gen_range(0..images.len());

                return Ok(images[index].to_string());
            } else {
                return Err(ServerKind::SettingValueNotFound.into());
            }
        } else {
            return Err(ServerKind::SettingValueNotFound.into());
        }
    }

    pub async fn create_user(&self, form: UserForm) -> ServerResult<User> {
        let encryptor = Encryptor::new(self.state.config.encrypt.secure.as_bytes());

        let uid = xid::new().to_string();

        let encrypt_data = encryptor.encode(&form.auth_data);

        let form = UserFormEncrypt::from_form(form, encrypt_data, &uid);

        let beign = self.state.conn.begin().await?;

        let storage = UserStorage::new(&beign);

        let user = storage.create_user(form).await?;

        beign.commit().await?;

        Ok(user)
    }
}
