use crate::schema::videos;

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