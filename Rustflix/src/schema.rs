table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        removed -> Bool,
    }
}

table! {
    videos (id) {
        id -> Int4,
        title -> Varchar,
        description -> Text,
        removed -> Bool,
    }
}

table! {
    views (id) {
        id -> Int4,
        video_id -> Int4,
        user_id -> Int4,
        watch_start -> Timestamp,
        duration -> Int4,
    }
}

joinable!(views -> users (user_id));
joinable!(views -> videos (video_id));

allow_tables_to_appear_in_same_query!(
    users,
    videos,
    views,
);
