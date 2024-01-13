use poem::web::Data;
use poem_openapi::{payload::Json, OpenApi};

use crate::{services::UserService, state::State, web::security::UserId};

use crate::web::response::{bad_response_handler, GenericApiResponse, ResponseObject};

mod dto;

pub use dto::*;

pub struct UserApi;

#[OpenApi(tag = "super::ApiTags::UserApi")]
impl UserApi {
    #[oai(path = "/user/add_article", method = "post")]
    async fn add_article(
        &self,
        state: Data<&State>,
        user_id: UserId,
        form: Json<ArticleFormRequest>,
    ) -> GenericApiResponse<ArticleResponse> {
        let service = UserService::new(&state);
        let form = form.get_form(user_id.0);

        match service.add_article(form).await {
            Err(e) => {
                return GenericApiResponse::Ok(Json(bad_response_handler(e)));
            }
            Ok(article) => {
                return GenericApiResponse::Ok(Json(ResponseObject::ok(
                    ArticleResponse::from_article(article),
                )));
            }
        }
    }

    #[oai(path = "/user/get_trend_list", method = "post")]
    async fn get_trend_list(
        &self,
        state: Data<&State>,
        user_id: UserId,
        params: Json<TrendParamsRequest>,
    ) -> GenericApiResponse<TrendListResponse> {
        let service = UserService::new(&state);
        let params = params.get_params(user_id.0);

        let page_size = params.page_size;
        let page = params.page;

        match service.get_trend_list(params).await {
            Err(e) => {
                return GenericApiResponse::Ok(Json(bad_response_handler(e)));
            }
            Ok(trends) => {
                let mut has_next = true;

                if trends.len() < page_size as usize {
                    has_next = false;
                }

                let res = TrendListResponse {
                    has_next,
                    data: trends
                        .into_iter()
                        .map(|trend| TrendDetailResponse::from_trend(trend))
                        .collect::<Vec<TrendDetailResponse>>(),
                    page: page_size,
                    page_size: page,
                };

                return GenericApiResponse::Ok(Json(ResponseObject::ok(res)));
            }
        }
    }

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
    ) -> GenericApiResponse<UserDetailResponse> {
        let service = UserService::new(&state);
        match service.get_user_info(user_id.0).await {
            Err(e) => {
                return GenericApiResponse::Ok(Json(bad_response_handler(e)));
            }
            Ok(user) => {
                return GenericApiResponse::Ok(Json(ResponseObject::ok(
                    UserDetailResponse::from_user(user),
                )));
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
    ) -> GenericApiResponse<UserDetailResponse> {
        let form = form.get_user_form();
        let service = UserService::new(&state);
        match service.create_user(form).await {
            Err(e) => {
                return GenericApiResponse::Ok(Json(bad_response_handler(e)));
            }
            Ok(user) => {
                return GenericApiResponse::Ok(Json(ResponseObject::ok(
                    UserDetailResponse::from_user(user),
                )));
            }
        }
    }
}
