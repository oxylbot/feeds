use redis::{self, Commands, Client};

pub fn insert_guild<D>(data: D, con: &mut redis::Connection) -> redis::RedisResult<()> {
    let _: () = con.set(data.key, data.value);
}

pub fn grab_conn() -> &mut redis::Connection {
    let client = Client::new("redis://127.0.0.1").unwrap();

    let mut con = client.get_connection();

    con
}
