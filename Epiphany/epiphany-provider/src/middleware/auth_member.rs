
use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::{StatusCode, HeaderValue},
    Json,
};
use epiphany_utls::{jwt_utils, warp_response};

use crate::service::redis_service;

pub struct AuthMember;

#[async_trait]
impl<B> FromRequest<B> for AuthMember
where
    B: Send,
{
    type Rejection = (StatusCode, Json<warp_response::WarpResponse<String>>);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let headers = req.headers();
        let token = headers.get("Authorization");
        if token.is_none() {
            return Err((
                StatusCode::OK,
                Json(warp_response::WarpResponse::build(Err(Some(String::from(
                    "request should be with access_token",
                ))))),
            ));
        }

        let jwt_claims = jwt_utils::decode_token(String::from(token.unwrap().to_str().unwrap()));
        
        if jwt_claims.is_err() {
            return Err((
                StatusCode::OK,
                Json(warp_response::WarpResponse::build(Err(Some(String::from(
                    "token decode error",
                ))))),
            ));
        }
        let company_id = jwt_claims.unwrap().id;
        let cache_token = redis_service::get_string(company_id.to_string().as_str());
        if cache_token.is_err() {
            return Err((
                StatusCode::OK,
                Json(warp_response::WarpResponse::build(Err(cache_token.err()))),
            ));
        }
        if cache_token.unwrap().is_none() {
            return Err((
                StatusCode::OK,
                Json(warp_response::WarpResponse::build(Err(Some(String::from(
                    "token is not exitst",
                ))))),
            ));
        }
        req.headers_mut().insert("header_id",  HeaderValue::from(company_id));
        Result::Ok(Self {})
    }
}
