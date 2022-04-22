use crate::args::{
    VideoCommand, 
    VideoSubcommand, 
    CreateVideo, 
    UpdateVideo, 
    DeleteEntity
};

use crate::models::{NewVideo, Video};
use crate::db::establish_connection;
use diesel::prelude::*;

pub fn handle_video_command(video: VideoCommand) {
    let command = video.command;
    match command {
        VideoSubcommand::Create(video) => {
            create_video(video);
        }
        VideoSubcommand::Update(video) => {
            update_video(video);
        }
        VideoSubcommand::Delete(delete_entity) => {
            delete_video(delete_entity);
        }
        VideoSubcommand::Show => {
            show_videos();
        }
    }
}

pub fn create_video(video: CreateVideo) {
    println!("Creating video: {:?}", video);
    use crate::schema::videos::dsl::*;

    let connection = establish_connection();
    let new_video = NewVideo {
        title: &video.title,
        description: &video.description,
        removed: false,
    };

    diesel::insert_into(videos)
        .values(&new_video)
        .execute(&connection)
        .expect("Error saving new video");
}

pub fn update_video(video: UpdateVideo) {
    println!("Updating video: {:?}", video);
    use crate::schema::videos::dsl::*;

    let connection = establish_connection();
    let db_video = Video {
        id: video.id,
        title: video.title,
        description: video.description,
        removed: false,
    };
    
    diesel::update(videos.find(video.id))
        .set(&db_video)
        .execute(&connection)
        .expect("Error updating video");
}

pub fn delete_video(video: DeleteEntity) {
    println!("Deleting video: {:?}", video);
    use crate::schema::videos::dsl::*;

    let connection = establish_connection();
    diesel::delete(videos.find(video.id))
        .execute(&connection)
        .expect("Error deleting video");
}

pub fn show_videos() {
    println!("Showing videos");
    use crate::schema::videos::dsl::*;

    let connection = establish_connection();
    let results = videos
        .filter(removed.eq(false))
        .load::<Video>(&connection)
        .expect("Error loading videos");

    println!("Displaying {} videos", results.len());
    for video in results {
        println!("{:?}", video);
    }
}