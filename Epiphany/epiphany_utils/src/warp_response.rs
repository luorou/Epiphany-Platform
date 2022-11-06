/** */
use serde::Serialize;
use std::option::Option;


#[derive(Clone, Debug, Serialize)]
pub struct WarpResponse<T> {
    pub code: Option<i64>,
    pub success: bool,
    pub message: Option<String>,
    pub data: Option<T>,
}


impl <T> WarpResponse<T> {
    pub fn build(result:Result<T,Option<String>>)-> Self {
        match result {
            Ok(t) => WarpResponse {
                code:Some(200),
                success: true, 
                message: Some(String::from("success")),
                data: Some(t),
            },
            Err(e) => WarpResponse {
                code:Some(-200),
                success: false, 
                message: unwrap_message(e),
                data: None,
            },
        }
   }
    pub fn build_with_code (result:Result<T,Option<String>>,code:i64)-> WarpResponse<T> {
        match result {
            Ok(t) => WarpResponse {
                code:Some(200),
                success: true,
                message: Some(String::from("success")),
                data: Some(t),
            },
            Err(e) => WarpResponse {
                code:Some(code),
                success: false,
                message: unwrap_message(e),
                data: None,
            },
        }
    }
}

fn unwrap_message(err:Option<String>)->Option<String>{
    match err {
        None =>  Some(String::from("failed")),
        Some(e) => { Some(String::from(e))}
    }
}
