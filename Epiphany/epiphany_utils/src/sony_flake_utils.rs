use snowflake::{ SnowflakeIdBucket};

pub fn generated_id()->i64{
   SnowflakeIdBucket::new(1, 1).get_id()
}