use crate::schema::users;
use crate::schema::videos;
use crate::schema::views;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub removed: bool,
}

#[derive(Queryable, Debug, AsChangeset)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub removed: bool,
}

#[derive(Insertable)]
#[table_name = "videos"]
pub struct NewVideo<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub removed: bool,
}

#[derive(Debug, Queryable, AsChangeset)]
pub struct Video {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub removed: bool,
}

#[derive(Insertable)]
#[table_name = "views"]
pub struct NewView<'a> {
    pub video_id: i32,
    pub user_id: i32,
    pub watch_start: &'a chrono::NaiveDateTime,
    pub duration: i32,
}

#[derive(Debug, Queryable)]
pub struct View {
    pub id: i32,
    pub user_id: i32,
    pub video_id: i32,
    pub watch_start: chrono::NaiveDateTime,
    pub duration: i32,
}

