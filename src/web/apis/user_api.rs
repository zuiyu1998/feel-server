use poem::web::Data;
use poem_openapi::{payload::Json, types::Any, Enum, Object, OpenApi};

use rc_storage::prelude::{
    AuthClass, Label, Trend, TrendForm, TrendMeta, User, UserForm, UserLabel, UserLoginForm,
};

use rc_storage::chrono::NaiveDateTime;

use crate::{services::UserService, state::State, web::security::UserId};

use crate::web::response::{bad_response_handler, GenericApiResponse, ResponseObject};

use serde::{Deserialize, Serialize};

pub struct UserApi;

#[derive(Enum, Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum AuthClassRequest {
    Email,
}

impl From<AuthClassRequest> for AuthClass {
    fn from(value: AuthClassRequest) -> Self {
        match value {
            AuthClassRequest::Email => AuthClass::Email,
        }
    }
}

#[derive(Debug, Deserialize, Object, Serialize)]
pub struct AddLabelFormRequest {
    pub lable_id: i32,
}

#[derive(Debug, Deserialize, Object, Serialize)]
pub struct TrendFormRequest {
    pub content: String,
}

impl TrendFormRequest {
    pub fn get_form(&self, user_id: i32) -> TrendForm {
        TrendForm {
            user_id,
            content: self.content.clone(),
            meta: None,
        }
    }
}

#[derive(Debug, Deserialize, Object, Serialize)]
pub struct UserLoginFormRequest {
    pub auth_class: AuthClassRequest,
    pub username: String,
    pub password: String,
}

impl UserLoginFormRequest {
    pub fn get_user_form(&self) -> UserLoginForm {
        UserLoginForm {
            auth_class: AuthClass::from(self.auth_class),
            auth_name: self.username.clone(),
            auth_data: self.password.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Object, Serialize)]
pub struct UserFormRequest {
    pub avatar: String,
    pub auth_class: AuthClassRequest,
    pub auth_name: String,
    pub auth_data: String,
}

impl UserFormRequest {
    pub fn get_user_form(&self) -> UserForm {
        UserForm {
            avatar: self.avatar.clone(),
            auth_class: AuthClass::from(self.auth_class),
            auth_name: self.auth_name.clone(),
            auth_data: self.auth_data.clone(),
        }
    }
}

#[derive(Debug, Object)]
pub struct UserLabelResponse {
    pub id: i32,
    pub user_id: i32,
    pub label_id: i32,
    pub sequence: i32,
    pub create_at: Any<NaiveDateTime>,
    pub update_at: Any<NaiveDateTime>,
}

impl UserLabelResponse {
    fn from_user_label(user_label: UserLabel) -> UserLabelResponse {
        UserLabelResponse {
            id: user_label.id,
            user_id: user_label.user_id,
            label_id: user_label.label_id,
            sequence: user_label.sequence,
            create_at: Any(user_label.create_at),
            update_at: Any(user_label.update_at),
        }
    }
}

#[derive(Debug, Object)]
pub struct LabelResponse {
    pub id: i32,
    pub user_id: i32,
    pub sequence: i32,
    pub create_at: Any<NaiveDateTime>,
    pub update_at: Any<NaiveDateTime>,
    pub description: String,
    pub name: String,
    pub effect: i64,
}

impl LabelResponse {
    fn from_label(label: Label) -> LabelResponse {
        LabelResponse {
            id: label.id,
            user_id: label.user_id,
            sequence: label.sequence,
            description: label.description,
            name: label.name,
            effect: label.effect,
            create_at: Any(label.create_at),
            update_at: Any(label.update_at),
        }
    }
}

#[derive(Debug, Object)]
pub struct TrendMetaResponse {
    pub source: String,
    pub source_id: i32,
}

impl TrendMetaResponse {
    fn from_meta(trend_meta: TrendMeta) -> TrendMetaResponse {
        TrendMetaResponse {
            source: trend_meta.source,
            source_id: trend_meta.source_id,
        }
    }
}

#[derive(Debug, Object)]
pub struct TrendResponse {
    pub id: i32,
    pub user_id: i32,
    pub content: String,
    pub meta: Option<TrendMetaResponse>,
    pub create_at: Any<NaiveDateTime>,
    pub update_at: Any<NaiveDateTime>,
    pub like_count: i32,
    pub unlike_count: i32,
}

impl TrendResponse {
    fn from_trend(trend: Trend) -> TrendResponse {
        TrendResponse {
            id: trend.id,
            user_id: trend.user_id,
            content: trend.content,
            like_count: trend.like_count,
            unlike_count: trend.unlike_count,
            create_at: Any(trend.create_at),
            update_at: Any(trend.update_at),
            meta: trend
                .meta
                .and_then(|meta| Some(TrendMetaResponse::from_meta(meta))),
        }
    }
}

#[derive(Debug, Object)]
pub struct UserResponse {
    pub id: i32,
    pub nikename: String,
    pub uid: String,
    pub avatar: String,
    pub create_at: Any<NaiveDateTime>,
    pub update_at: Any<NaiveDateTime>,
}

impl UserResponse {
    fn from_user(user: User) -> UserResponse {
        UserResponse {
            id: user.id,
            nikename: user.nikename,
            uid: user.uid,
            avatar: user.avatar,
            create_at: Any(user.create_at),
            update_at: Any(user.update_at),
        }
    }
}

#[OpenApi(tag = "super::ApiTags::UserApi")]
impl UserApi {
    #[oai(path = "/user/add_trend", method = "post")]
    async fn add_trend(
        &self,
        state: Data<&State>,
        user_id: UserId,
        form: Json<TrendFormRequest>,
    ) -> GenericApiResponse<TrendResponse> {
        let service = UserService::new(&state);
        let form = form.get_form(user_id.0);

        match service.add_trend(form).await {
            Err(e) => {
                return GenericApiResponse::Ok(Json(bad_response_handler(e)));
            }
            Ok(trend) => {
                return GenericApiResponse::Ok(Json(ResponseObject::ok(
                    TrendResponse::from_trend(trend),
                )));
            }
        }
    }
    #[oai(path = "/user/add_label", method = "post")]
    async fn add_label(
        &self,
        state: Data<&State>,
        user_id: UserId,
        form: Json<AddLabelFormRequest>,
    ) -> GenericApiResponse<UserLabelResponse> {
        let service = UserService::new(&state);
        match service.add_label(user_id.0, form.lable_id).await {
            Err(e) => {
                return GenericApiResponse::Ok(Json(bad_response_handler(e)));
            }
            Ok(user) => {
                return GenericApiResponse::Ok(Json(ResponseObject::ok(
                    UserLabelResponse::from_user_label(user),
                )));
            }
        }
    }

    #[oai(path = "/user/get_user_label_list", method = "get")]
    async fn get_user_label_list(
        &self,
        state: Data<&State>,
        user_id: UserId,
    ) -> GenericApiResponse<Vec<LabelResponse>> {
        let service = UserService::new(&state);
        match service.get_user_label_list(user_id.0).await {
            Err(e) => {
                return GenericApiResponse::Ok(Json(bad_response_handler(e)));
            }
            Ok(labels) => {
                return GenericApiResponse::Ok(Json(ResponseObject::ok(
                    labels
                        .into_iter()
                        .map(|label| LabelResponse::from_label(label))
                        .collect::<Vec<LabelResponse>>(),
                )));
            }
        }
    }

    #[oai(path = "/user/get_user_info", method = "get")]
    async fn get_user_info(
        &self,
        state: Data<&State>,
        user_id: UserId,
    ) -> GenericApiResponse<UserResponse> {
        let service = UserService::new(&state);
        match service.get_user_info(user_id.0).await {
            Err(e) => {
                return GenericApiResponse::Ok(Json(bad_response_handler(e)));
            }
            Ok(user) => {
                return GenericApiResponse::Ok(Json(ResponseObject::ok(UserResponse::from_user(
                    user,
                ))));
            }
        }
    }

    #[oai(path = "/user/login", method = "post")]
    async fn login(
        &self,
        form: Json<UserLoginFormRequest>,
        state: Data<&State>,
    ) -> GenericApiResponse<String> {
        let form = form.get_user_form();

        let service = UserService::new(&state);
        match service.login(form).await {
            Err(e) => {
                return GenericApiResponse::Ok(Json(bad_response_handler(e)));
            }
            Ok(token) => {
                return GenericApiResponse::Ok(Json(ResponseObject::ok(token)));
            }
        }
    }

    #[oai(path = "/user/get_range_avatar", method = "get")]
    async fn get_range_avatar(&self, state: Data<&State>) -> GenericApiResponse<String> {
        let service = UserService::new(&state);
        match service.get_range_avatar().await {
            Err(e) => {
                return GenericApiResponse::Ok(Json(bad_response_handler(e)));
            }
            Ok(avatar) => {
                return GenericApiResponse::Ok(Json(ResponseObject::ok(avatar)));
            }
        }
    }

    #[oai(path = "/user/create_user", method = "post")]
    async fn create_user(
        &self,
        state: Data<&State>,
        form: Json<UserFormRequest>,
    ) -> GenericApiResponse<UserResponse> {
        let form = form.get_user_form();
        let service = UserService::new(&state);
        match service.create_user(form).await {
            Err(e) => {
                return GenericApiResponse::Ok(Json(bad_response_handler(e)));
            }
            Ok(user) => {
                return GenericApiResponse::Ok(Json(ResponseObject::ok(UserResponse::from_user(
                    user,
                ))));
            }
        }
    }
}
