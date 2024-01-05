use crate::{encryptor::Encryptor, state::State, ServerResult};

use rc_entity::sea_orm::TransactionTrait;
use rc_storage::prelude::{User, UserForm, UserFormEncrypt, UserStorage};

pub struct UserService<'a> {
    state: &'a State,
}

impl<'a> UserService<'a> {
    pub async fn create_user(&self, form: UserForm) -> ServerResult<User> {
        let encryptor = Encryptor::new(self.state.config.encrypt.secure.as_bytes());

        let encrypt_data = encryptor.encode(&form.auth_data);

        let form = UserFormEncrypt::from_form(form, encrypt_data);

        let beign = self.state.conn.begin().await?;

        let storage = UserStorage::new(&beign);

        let user = storage.create_user(form).await?;

        beign.commit().await?;

        Ok(user)
    }
}
