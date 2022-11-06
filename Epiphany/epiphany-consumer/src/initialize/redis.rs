use redis::{ConnectionInfo, ConnectionAddr, RedisConnectionInfo};

use crate::APP_CONTEXT;

pub async fn init_redis(){
    let conn_info = ConnectionInfo {
        addr: ConnectionAddr::Tcp("47.100.57.61".to_string(), 6379),
        redis:RedisConnectionInfo{
            db:0,
            username:None,
            password:Some(String::from("thankgod"))
        }
    };
    let client = redis::Client::open(conn_info).expect("redis open failed");
    APP_CONTEXT.set::<redis::Client>(client);
}

