use crate::rustflix_args::{
    ViewSubcommand,
    ViewCommand, 
    CreateView,
};
use rustflix_diesel::establish_connection;
use rustflix_diesel::models::{
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
    use rustflix_diesel::schema::views::dsl::*;

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

    use rustflix_diesel::schema::views::dsl::*;

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

    use rustflix_diesel::schema::views::dsl::*;

    let connection = establish_connection();

    let results = views
        .load::<DBView>(&connection)
        .expect("Error loading views");

    for view in results {
        println!("{:?}", view);
    }
}