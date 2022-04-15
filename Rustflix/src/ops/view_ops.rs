use crate::args::{
    ViewSubcommand,
    ViewCommand, 
    CreateView,
};
use crate::db::establish_connection;
use crate::models::{
    View as DBView, 
    NewView
};
use diesel::prelude::*;

pub fn handle_view_command(view: ViewCommand) {
    let command = view.command;
    match command {
        ViewSubcommand::Create(new_view) => {
            create_view(new_view);
        }
        ViewSubcommand::Show => {
            show_views();
        }
        ViewSubcommand::ShowPretty => {
            show_views_pretty();
        }
    }
}

fn create_view(new_view: CreateView) {
    println!("Creating view: {:?}", new_view);
    use crate::schema::views::dsl::*;

    let connection = establish_connection();
    let new_view = NewView {
        user_id: new_view.user_id,
        video_id: new_view.video_id,
        watch_start: &new_view.watch_start,
        duration: new_view.duration,
    };

    diesel::insert_into(views)
        .values(&new_view)
        .execute(&connection)
        .expect("Error saving new view");
}

fn show_views() {
    println!("Showing views");

    use crate::schema::views::dsl::*;

    let connection = establish_connection();

    let results = views
        .load::<DBView>(&connection)
        .expect("Error loading views");

    println!("Displaying {} views", results.len());
    for view in results {
        println!("{:?}", view);
    }
}

fn show_views_pretty() {
    println!("Showing views");

    use crate::schema::views;
    use crate::schema::videos;
    use crate::schema::users;

    let connection = establish_connection();

    let results = views::table
        .inner_join(videos::table)
        .inner_join(users::table)
        .select((
            users::name,
            videos::title,
            views::watch_start,
            views::duration
        ))
        .load::<(String, String, chrono::NaiveDateTime, i32)>(&connection)
        .expect("Error loading views");

    for view in results {
        println!("{:?} {:?} {:?} {:?}", view.0, view.1, view.2, view.3);
    }
}