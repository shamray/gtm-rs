use axum::{extract::Request, middleware::Next, response::Response};
use tower_cookies::Cookies;
use tracing::info;

use crate::{Error, Result, web};

pub async fn mw_require_auth(cookies: Cookies, request: Request, next: Next) -> Result<Response> {
    info!("{:?}", cookies);
    let auth_token = cookies.get(web::AUTH_TOKEN).map(|c| c.value().to_string());

    auth_token.ok_or(Error::AuthFailNoAuthTokenCookie)?;
    Ok(next.run(request).await)
}
