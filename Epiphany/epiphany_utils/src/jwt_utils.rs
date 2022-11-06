use chrono::Local;
use jsonwebtoken::{Header, encode, EncodingKey, decode, DecodingKey, Validation};
use serde::{Serialize,Deserialize};

#[derive(Clone, Debug, Serialize,Deserialize)]
pub struct JWTClaims{
    pub id: i64,
    pub exp: i64,
}

pub fn create_token(id:i64)->Result<String,Option<String>>{
    let claims = JWTClaims{
        id,
        exp:Local::now().timestamp_millis()
    };
    let  header = Header::new(jsonwebtoken::Algorithm::HS256);
    let token = encode(&header, &claims, &EncodingKey::from_secret("secret".as_ref()));
    match token {
        Ok(t) => Result::Ok(t),
        Err(e) => Result::Err(Some(e.to_string())),
    }
}

pub fn decode_token(token:String)->Result<JWTClaims,Option<String>>{
    let token_data = 
    decode::<JWTClaims>(&token.as_str(), 
    &DecodingKey::from_secret("secret".as_ref()), &Validation::default());
    match token_data {
        Ok(td) => Result::Ok(td.claims),
        Err(e) => Result::Err(Some(e.to_string())),
    }
}