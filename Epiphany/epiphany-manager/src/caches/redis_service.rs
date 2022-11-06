use std::time::Duration;


use crate::APP_CONTEXT;

pub  fn get_redis_conn() -> Result<redis::Connection,String> {
    let client = APP_CONTEXT.get::<redis::Client>();
    let conn = client.get_connection();
    match conn {
        Ok(c) => Result::Ok(c),
        Err(e) => Result::Err(e.to_string()),
    }
}


pub  fn set_string(k:&str, v: &str)-> Result<Option<String>,String>
{
    let   conn = get_redis_conn();
    if conn.is_err() {
        return Result::Err(String::from("get redis conn error"));
    }
    match redis::cmd("SET").arg(&[k, v]).query::<Option<String>>( &mut conn.unwrap()) {
        Ok(rs) => Result::Ok(rs),
        Err(e) => Result::Err(e.to_string()),
    }
}

pub  fn set_string_ex(k:&str, v: &str, ex: Option<Duration>)-> Result<String,String>
{
    let   conn = get_redis_conn();
    if conn.is_err() {
        return Result::Err(String::from("get redis conn error"));
    }
    match redis::cmd("SET").arg(&[k, v,&ex.unwrap().as_secs().to_string()]).query::<String>( &mut conn.unwrap()) {
        Ok(rs) => Result::Ok(rs),
        Err(e) => Result::Err(e.to_string()),
    }
}

pub fn get_string(k:&str)->Result<Option<String>, String>{
    let   conn = get_redis_conn();
    if conn.is_err() {
        return Result::Err(String::from("get redis conn error"));
    }
    match redis::cmd("GET").arg(&[k]).query::<Option<String>>(&mut conn.unwrap()) {
        Ok(v) => Result::Ok(v),
        Err(e) => Result::Err(e.to_string()),
    }
    
      
}