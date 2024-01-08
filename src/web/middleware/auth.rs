use poem::{
    async_trait,
    http::{header, HeaderName},
    Body, Endpoint, IntoResponse, Middleware, Request, Response, Result,
};
use poem_openapi::types::ToJSON;

use crate::{
    jwt_helper::JwtHelper, state::State, web::response::bad_response_handler, ServerResult,
};

use super::MiddlewareKind;

#[derive(Debug, Clone)]
pub struct Auth {
    header_name: HeaderName,
    scheme: String,
}

impl<E: Endpoint> Middleware<E> for Auth {
    type Output = AuthImpl<E>;

    fn transform(&self, ep: E) -> Self::Output {
        AuthImpl {
            ep,
            auth: self.clone(),
        }
    }
}

impl Auth {
    pub fn bearer() -> Self {
        Auth {
            header_name: header::AUTHORIZATION,
            scheme: "bearer".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token(String);

#[derive(Debug, Clone)]
pub struct UserId(i32);

impl Auth {
    pub fn parse(&self, req: &Request) -> ServerResult<Token> {
        let header_value = req
            .header(&self.header_name)
            .ok_or(MiddlewareKind::HeaderValueNotFound)?;

        let mut spilt_n = header_value.splitn(2, ' ');

        match spilt_n.next() {
            Some(scheme) if self.scheme == scheme => {}
            _ => {
                return Err(MiddlewareKind::HeaderValueScheme.into());
            }
        }

        match spilt_n.next() {
            None => {
                return Err(MiddlewareKind::HeaderValueScheme.into());
            }
            Some(raw) => {
                return Ok(Token(raw.to_string()));
            }
        }
    }
}

pub struct AuthImpl<E: Endpoint> {
    ep: E,
    auth: Auth,
}

impl<E: Endpoint> AuthImpl<E> {
    pub async fn call_(&self, req: &mut Request) -> ServerResult<()> {
        let token = self.auth.parse(req)?;

        let state = req.data::<State>().unwrap();

        let jwt_helper = JwtHelper::from_config(&state.config.jwt);

        let user_id = jwt_helper.decode(&token.0)?;

        let user_id = user_id.parse::<i32>().unwrap();

        let user_id = UserId(user_id);

        req.extensions_mut().insert(token);
        req.extensions_mut().insert(user_id);

        Ok(())
    }
}

#[async_trait]
impl<E: Endpoint> Endpoint for AuthImpl<E> {
    type Output = Response;

    async fn call(&self, mut req: Request) -> Result<Self::Output> {
        if let Err(e) = self.call_(&mut req).await {
            let res = bad_response_handler::<String>(e);
            let json = res.to_json().unwrap();
            let body = Body::from_json(json).unwrap();

            return Ok(body.into());
        }

        let output = self.ep.call(req).await?;

        Ok(output.into_response())
    }
}
