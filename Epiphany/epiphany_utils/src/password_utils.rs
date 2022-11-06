
use balloon_hash::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Balloon
};
use sha2::Sha256;

/**
 * 
 */
pub fn password_balloon_scrypt(password:String) -> Result<String,Option<String>>{
    let salt = SaltString::generate(&mut OsRng);
    let balloon = Balloon::<Sha256>::default();
    let pwd_hash = balloon.hash_password(password.as_bytes(), &salt);
    match pwd_hash {
        Ok(pwd) => Result::Ok(pwd.to_string()),
        Err(_) => Result::Err(Some(String::from("password scrypt failed"))),
    }
}
/**
 * 
 */
pub fn password_balloon_eq(password:String,password_scrypt:String) ->bool{
    let balloon = Balloon::<Sha256>::default();
    let parsed_hash= PasswordHash::new(password_scrypt.as_str());
    match parsed_hash {
        Ok(ph) => balloon.verify_password(password.as_bytes(), &ph).is_ok(),
        Err(_) => false,
    }
    
}