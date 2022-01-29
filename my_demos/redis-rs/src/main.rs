extern crate redis;
use redis::Commands;

fn main() {
    fetch_an_integer();
}


fn fetch_an_integer() -> redis::RedisResult<isize> {
    //connect to redis
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut conn = client.get_connection()?;
    let _ : () = conn.set("my_key",42)?;
    return conn.get("my_key")
}

