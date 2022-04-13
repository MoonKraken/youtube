mod rustflix_args;
mod user_ops;
mod video_ops;
mod view_ops;

use user_ops::handle_user_command;
use video_ops::handle_video_command;
use view_ops::handle_view_command;
use rustflix_diesel::establish_connection;
use rustflix_args::RustflixArgs;
use clap::Parser;

fn main() {
    let args = RustflixArgs::parse();

    match args.data_type {
        rustflix_args::DataType::User(user) => handle_user_command(user),
        rustflix_args::DataType::Video(video) => handle_video_command(video),
        rustflix_args::DataType::View(view) => handle_view_command(view),
    };
}