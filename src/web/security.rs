use poem::Request;
use poem_openapi::{auth::Bearer, SecurityScheme};

use crate::{helper::JwtHelper, state::State};

#[derive(SecurityScheme)]
#[oai(
    ty = "bearer",
    key_name = "authorization",
    key_in = "header",
    checker = "api_checker"
)]
pub struct UserId(pub i32);

async fn api_checker(req: &Request, bearer: Bearer) -> Option<i32> {
    let state = req.data::<State>()?;
    let helper = JwtHelper::from_config(&state.config.jwt);

    let user_id = helper.decode(&bearer.token).ok()?;
    let user_id = user_id.parse::<i32>().ok()?;

    Some(user_id)
}
