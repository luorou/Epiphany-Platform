use axum::http::HeaderMap;

pub fn get_header_id(headers: HeaderMap, key: &str) -> Result<i64, Option<String>> {
    let v = headers.get(key);
    if v.is_none() {
        return Result::Err(Some(String::from("header id is not exitst")));
    }
    let vstr = v.unwrap().to_str();

    if vstr.is_err() {
        return  Result::Err(Some(vstr.err().unwrap().to_string()));
    }
    let hidstr = vstr.unwrap().to_string();
    
    match hidstr.parse::<i64>() {
        Ok(v) => {Result::Ok(v)},
        Err(e) => {Result::Err(Some(e.to_string()))},
    }
}
