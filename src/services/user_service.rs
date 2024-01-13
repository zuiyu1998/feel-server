use crate::{encryptor::Encryptor, jwt_helper::JwtHelper, state::State, ServerKind, ServerResult};

use rand::{thread_rng, Rng};
use rc_entity::sea_orm::TransactionTrait;
use rc_storage::prelude::{
    Article, ArticleForm, ArticleStorage, FollowStorage, Label, LabelStorage, RelatedThrend, Trend,
    TrendDetail, TrendForm, TrendParams, TrendStorage, User, UserFollowDetailForm, UserFollowForm,
    UserForm, UserFormEncrypt, UserLabel, UserLoginForm, UserLoginFormEncrypt, UserStorage,
};

pub struct UserService<'a> {
    state: &'a State,
}

impl<'a> UserService<'a> {
    pub fn new(state: &'a State) -> Self {
        UserService { state }
    }

    pub async fn add_article(&self, form: ArticleForm) -> ServerResult<Article> {
        let beign = self.state.conn.begin().await?;
        let storage = ArticleStorage::new(&beign);

        let article = storage.create_article(form).await?;
        let trend_from = article.get_thrend_form();

        let trend_storage = TrendStorage::new(&beign);

        trend_storage.create_trend(trend_from).await?;

        beign.commit().await?;

        Ok(article)
    }

    pub async fn get_trend_list(&self, params: TrendParams) -> ServerResult<Vec<TrendDetail>> {
        let beign = self.state.conn.begin().await?;
        let storage = TrendStorage::new(&beign);
        let trends = storage.get_list(params).await?;
        beign.commit().await?;

        Ok(trends)
    }

    pub async fn add_trend(&self, form: TrendForm) -> ServerResult<Trend> {
        let beign = self.state.conn.begin().await?;
        let storage = TrendStorage::new(&beign);

        let trend = storage.create_trend(form).await?;

        beign.commit().await?;

        Ok(trend)
    }

    pub async fn add_label(&self, user_id: i32, label_id: i32) -> ServerResult<UserLabel> {
        let beign = self.state.conn.begin().await?;
        let storage = LabelStorage::new(&beign);

        let user_label = storage.add_label(user_id, label_id).await?;

        beign.commit().await?;

        Ok(user_label)
    }

    pub async fn get_user_label_list(&self, user_id: i32) -> ServerResult<Vec<Label>> {
        let beign = self.state.conn.begin().await?;
        let storage = LabelStorage::new(&beign);

        let labels = storage.get_user_label_list(user_id).await?;
        beign.commit().await?;

        Ok(labels)
    }

    pub async fn get_user_info(&self, user_id: i32) -> ServerResult<User> {
        let beign = self.state.conn.begin().await?;

        let storage = UserStorage::new(&beign);

        let user = storage.find(user_id).await?;

        beign.commit().await?;

        Ok(user)
    }

    pub async fn login(&self, form: UserLoginForm) -> ServerResult<String> {
        let encryptor = Encryptor::new(self.state.config.encrypt.secure.as_bytes());
        let encrypt_data = encryptor.encode(&form.auth_data);

        let form = UserLoginFormEncrypt::from_form(form, encrypt_data);
        let beign = self.state.conn.begin().await?;

        let storage = UserStorage::new(&beign);

        let user = storage.login(form).await?;

        beign.commit().await?;

        let jwt_helper = JwtHelper::from_config(&self.state.config.jwt);

        let token = jwt_helper.encode(&user.id.to_string())?;

        Ok(token)
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

        let follow_storage = FollowStorage::new(&beign);

        let detail = follow_storage.create_user_follow(user.id).await?;

        beign.commit().await?;

        Ok(user)
    }
}
