use leptos::{use_context, Scope};
use std::thread;
use std::collections::HashMap;
use std::io::prelude::*;
use leptos::*;
use cfg_if::cfg_if;
use redis::Commands;
use redis::Script;
use leptos::ServerFnError::ServerError;
use std::fs::File;
use std::env;
use reqwest::{Client};

cfg_if! {
    if #[cfg(feature = "ssr")] {
        pub fn register_server_functions() {
            _ = GetEntry::register();
            _ = CreateEntry::register();
            _ = GetEntries::register();
        }

        pub async fn redis_client() -> redis::Client {
            let password = env::var("REDISCLI_AUTH").unwrap();
            let redis_connection_string = format!(
                "redis://default:{}@tolerant-parakeet-32706.upstash.io:32706",
                password
            );
            redis::Client::open(redis_connection_string).unwrap()
        }
    }
}

async fn log_to_parseable(userid: &str, date: &str, action: &str) {
    const PARSEABLE_URL: &str = "http://localhost:8000/api/v1/logstream/journalapp";
    let mut map = HashMap::new();
    map.insert("userid", userid);
    map.insert("date", date);
    map.insert("action", action);

    let client = Client::new();

    client
        .post(PARSEABLE_URL)
        .header("Authorization", "Basic YWRtaW46YWRtaW4=")
        .header("Content-Type", "application/json")
        .json(&map)
        .send()
        .await
        .unwrap();
}

#[server(GetEntry, "/api")]
pub async fn get_entry(cx: Scope, userid: String, date: String) -> Result<String, ServerFnError> {
    // thread::spawn(|| {
        log_to_parseable(&userid, &date, "get_entry").await;
    // })

    let mut con = redis_client().await.get_connection().unwrap();
    let key = format!("{}:{}", userid, date);
    let result = con.get(key).unwrap();
    dbg!(&result);
    Ok(result)
}

#[server(CreateEntry "/api")]
pub async fn create_entry(cx: Scope, userid: String, date: String, content: String) -> Result<(), ServerFnError> {
    let mut con = redis_client().await.get_connection().unwrap();
    let key = format!("{}:{}", userid, date);
    let _ : () = con.set(key, content).unwrap();

    let zadd_key = format!("{}:entries", userid);
    let _ : () = con.zadd(zadd_key, date, 0).unwrap();
    Ok(())
}

#[server(GetEntries "/api")]
pub async fn get_entries(cx: Scope, userid: String, start_date: String, end_date: String) -> Result<Vec<String>, ServerFnError> {
    let mut con = redis_client().await.get_connection().unwrap();
    let mut script_file = File::open("get_entries.lua").unwrap();
    let mut contents = String::new();
    script_file.read_to_string(&mut contents).unwrap();
    let script = Script::new(contents.as_str());
    let res = script.arg(userid).arg(start_date).arg(end_date).invoke(&mut con);
    dbg!(&res);
    Ok(res.unwrap_or(vec![]))
}
