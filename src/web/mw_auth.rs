use axum::{
    RequestPartsExt,
    extract::{FromRequestParts, Request},
    http::request::Parts,
    middleware::Next,
    response::Response,
};
use lazy_regex::regex_captures;
use tower_cookies::Cookies;
use tracing::info;

use crate::{Error, Result, ctx::Ctx, web};

pub async fn mw_require_auth(ctx: Result<Ctx>, request: Request, next: Next) -> Result<Response> {
    ctx?;

    Ok(next.run(request).await)
}

fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, id, exp, sign) = regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)$"#, &token)
        .ok_or(Error::AuthFailTokenWrongFormat)?;

    let id: u64 = id.parse().map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((id, exp.to_string(), sign.to_string()))
}

impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        info!("{:12} - Ctx", "EXTRACTOR");

        let cookies = parts.extract::<Cookies>().await.unwrap();
        info!("{:?}", cookies);

        let auth_token = cookies.get(web::AUTH_TOKEN).map(|c| c.value().to_string());

        let (user_id, _exp, _sign) = auth_token
            .ok_or(Error::AuthFailNoAuthTokenCookie)
            .and_then(parse_token)?;

        Ok(Ctx::new(user_id))
    }
}
