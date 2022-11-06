
use axum::{
    async_trait,
    extract::{FromRequest, RequestParts}, http::StatusCode, Json,
};
use epiphany_utls::{warp_response, jwt_utils};
use crate::caches;

pub struct AuthAdmin;

#[async_trait]
impl<B> FromRequest<B> for AuthAdmin
where
    B: Send,
{
    type Rejection = (StatusCode,Json<warp_response::WarpResponse<String>>);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let headers = req.headers();
        let token = headers.get("authorization");
        if token.is_none() {
            return Err((
                StatusCode::OK,
                Json(warp_response::WarpResponse::build(Err(Some(String::from("request should be with authorization")))))
            ))
        }

       let jwt_claims =  jwt_utils::decode_token(String::from(token.unwrap().to_str().unwrap()));
       if jwt_claims.is_err() {
        return Err((
            StatusCode::OK,
            Json(warp_response::WarpResponse::build(Err(Some(String::from("token decode error")))))
        ));
       }
       
     let cache_token =   
        caches::redis_service::get_string(jwt_claims.unwrap().id.to_string().as_str());        
     if cache_token.is_err() {
        return Err((
            StatusCode::OK,
            Json(warp_response::WarpResponse::build(Err(cache_token.err())))
        ));
       }
    if cache_token.unwrap().is_none() {
        return Err((
            StatusCode::OK,
            Json(warp_response::WarpResponse::build(Err(Some(String::from("token is not exitst")))))
        ));
    }
       Result::Ok(Self{})
    }
}

