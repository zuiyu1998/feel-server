use poem::web::Data;
use poem_openapi::types::Any;
use poem_openapi::{payload::Json, Object, OpenApi};
use serde::{Deserialize, Serialize};

use rc_storage::chrono::NaiveDateTime;
use rc_storage::prelude::{LabelTemplate, LableForm};

use crate::{
    services::LabelService,
    state::State,
    web::response::{bad_response_handler, GenericApiResponse, ResponseObject},
};

#[derive(Debug, Object)]
pub struct LabelTemplateResponse {
    pub id: i32,
    pub description: String,
    pub name: String,
    pub effect: i64,
    pub create_at: Any<NaiveDateTime>,
    pub update_at: Any<NaiveDateTime>,
}

impl LabelTemplateResponse {
    pub fn from_template(template: LabelTemplate) -> Self {
        LabelTemplateResponse {
            id: template.id,
            description: template.description,
            name: template.name,
            effect: template.effect,
            create_at: Any(template.create_at),
            update_at: Any(template.update_at),
        }
    }
}

#[derive(Debug, Deserialize, Object, Serialize)]
pub struct LableFormRequest {
    pub description: String,
    pub name: String,
    pub effect: i64,
}

impl LableFormRequest {
    pub fn get_label_form(&self) -> LableForm {
        LableForm {
            description: self.description.clone(),
            name: self.name.clone(),
            effect: self.effect.clone(),
        }
    }
}

pub struct LabelApi;

#[OpenApi(tag = "super::ApiTags::LabelApi")]
impl LabelApi {
    #[oai(path = "/user/get_user_info", method = "post")]
    async fn get_user_info(
        &self,
        state: Data<&State>,
        form: Json<LableFormRequest>,
    ) -> GenericApiResponse<LabelTemplateResponse> {
        let form = form.get_label_form();

        let service = LabelService::new(&state);
        match service.create_label(form).await {
            Err(e) => {
                return GenericApiResponse::Ok(Json(bad_response_handler(e)));
            }
            Ok(template) => {
                return GenericApiResponse::Ok(Json(ResponseObject::ok(
                    LabelTemplateResponse::from_template(template),
                )));
            }
        }
    }
}
