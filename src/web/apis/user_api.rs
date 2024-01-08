use poem::{web::Data, Error};
use poem_openapi::{
    payload::Json,
    types::{Any, ParseFromJSON, ToJSON},
    ApiResponse, Enum, Object, OpenApi,
};

use rc_storage::prelude::{AuthClass, User, UserForm, UserLoginForm};

use rc_storage::chrono::NaiveDateTime;

use crate::{services::UserService, state::State};

use super::response::{bad_request_handler, bad_response_handler, ResponseObject};

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

#[derive(ApiResponse)]
#[oai(bad_request_handler = "inline_bad_request_handler")]
enum UserApiResponse<T: ParseFromJSON + ToJSON + Send + Sync> {
    #[oai(status = 200)]
    Ok(Json<ResponseObject<T>>),
}

fn inline_bad_request_handler<T: ParseFromJSON + ToJSON + Send + Sync>(
    err: Error,
) -> UserApiResponse<T> {
    UserApiResponse::Ok(Json(bad_request_handler(err)))
}

#[OpenApi(tag = "super::ApiTags::UserApi")]
impl UserApi {
    #[oai(path = "/user/login", method = "post")]
    async fn login(
        &self,
        form: Json<UserLoginFormRequest>,
        state: Data<&State>,
    ) -> UserApiResponse<String> {
        let form = form.get_user_form();

        let service = UserService::new(&state);
        match service.login(form).await {
            Err(e) => {
                return UserApiResponse::Ok(Json(bad_response_handler(e)));
            }
            Ok(token) => {
                return UserApiResponse::Ok(Json(ResponseObject::ok(token)));
            }
        }
    }

    #[oai(path = "/user/get_range_avatar", method = "get")]
    async fn get_range_avatar(&self, state: Data<&State>) -> UserApiResponse<String> {
        let service = UserService::new(&state);
        match service.get_range_avatar().await {
            Err(e) => {
                return UserApiResponse::Ok(Json(bad_response_handler(e)));
            }
            Ok(avatar) => {
                return UserApiResponse::Ok(Json(ResponseObject::ok(avatar)));
            }
        }
    }

    #[oai(path = "/user/create_user", method = "post")]
    async fn create_user(
        &self,
        state: Data<&State>,
        form: Json<UserFormRequest>,
    ) -> UserApiResponse<UserResponse> {
        let form = form.get_user_form();
        let service = UserService::new(&state);
        match service.create_user(form).await {
            Err(e) => {
                return UserApiResponse::Ok(Json(bad_response_handler(e)));
            }
            Ok(user) => {
                return UserApiResponse::Ok(Json(ResponseObject::ok(UserResponse::from_user(
                    user,
                ))));
            }
        }
    }
}
