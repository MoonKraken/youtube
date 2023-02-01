use leptos::{use_context, Scope};
use std::io::prelude::*;
use leptos::*;
use cfg_if::cfg_if;
use redis::Commands;
use redis::Script;
use leptos::ServerFnError::ServerError;
use std::fs::File;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        pub fn register_server_functions() {
            _ = GetEntry::register();
            _ = PutEntry::register();
            _ = GetEntries::register();
        }

        pub async fn redis_client() -> redis::Client {
            redis::Client::open("redis://default:c6033e870d29415ab1b8ab96ba158b71@usw2-relative-tahr-30262.upstash.io:30262").unwrap()
        }
    }
}

#[server(GetEntry, "/api")]
pub async fn get_entry(cx: Scope, userid: String, date: String) -> Result<String, ServerFnError> {
    let mut con = redis_client().await.get_connection().unwrap();
    let key = format!("{}:{}", userid, date);
    let result = con.get(key).unwrap();
    dbg!(&result);
    Ok(result)
}

#[server(PutEntry "/api")]
pub async fn put_entry(cx: Scope, userid: String, date: String, content: String) -> Result<(), ServerFnError> {
    // this is just an example of how to access server context injected in the handlers
    let mut con = redis_client()
        .await
        .get_connection()
        .unwrap();

    let set_key = format!("{}:{}", userid, date);

    //need to add the date to the sorted set of the user's entries
    // as well as the contents of the entry
    // maybe this should be made into an atomic set of operations?
    let _ : () = con.set(set_key, content).unwrap();

    let zadd_key = format!("{}:entries", userid);
    let _ : () = con.zadd(zadd_key, date, 0).unwrap();
    Ok(())
}

#[server(GetEntries "/api")]
pub async fn get_entries(cx: Scope, userid: String, start_date: String, end_date: String) -> Result<Vec<String>, ServerFnError> {
    let mut con = redis_client()
        .await
        .get_connection()
        .unwrap();

    //load lua script from file to string
    let mut script_file = File::open("get_entries.lua").unwrap();
    let mut contents = String::new();
    script_file.read_to_string(&mut contents).unwrap();
    dbg!(&contents);
    let script = Script::new(contents.as_str());
    let res = script.arg(userid).arg(start_date).arg(end_date).invoke(&mut con);
    dbg!(&res);

    Ok(res.unwrap_or(vec![]))
}
