use poem::{web::Data, Error};
use poem_openapi::{
    payload::Json,
    types::{Any, ParseFromJSON, ToJSON},
    ApiResponse, Enum, Object, OpenApi,
};

use rc_storage::prelude::{AuthClass, User, UserForm};

use crate::{services::UserService, state::State};

use super::response::{bad_request_handler, bad_response_handler, ResponseObject};

use serde::{Deserialize, Serialize};

pub struct Api;

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
pub struct UserFormRequest {
    pub nikename: String,
    pub avatar: String,
    pub auth_class: AuthClassRequest,
    pub auth_name: String,
    pub auth_data: String,
}

impl UserFormRequest {
    pub fn get_user_form(&self) -> UserForm {
        UserForm {
            nikename: self.nikename.clone(),
            avatar: self.avatar.clone(),
            auth_class: AuthClass::from(self.auth_class),
            auth_name: self.auth_name.clone(),
            auth_data: self.auth_data.clone(),
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
impl Api {
    #[oai(path = "/user/create_user", method = "post")]
    async fn create_user(
        &self,
        state: Data<&State>,
        form: Json<UserFormRequest>,
    ) -> UserApiResponse<Any<User>> {
        let form = form.get_user_form();
        let service = UserService::new(&state);
        match service.create_user(form).await {
            Err(e) => {
                return UserApiResponse::Ok(Json(bad_response_handler(e)));
            }
            Ok(user) => {
                return UserApiResponse::Ok(Json(ResponseObject::ok(Any(user))));
            }
        }
    }
}
