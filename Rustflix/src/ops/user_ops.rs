use crate::args::{
    UserSubcommand, 
    UserCommand, 
    CreateUser, 
    UpdateUser, 
    DeleteEntity
};
use crate::db::establish_connection;
use crate::models::{NewUser, User as DBUser};
use diesel::prelude::*;

pub fn handle_user_command(user: UserCommand) {
    let command = user.command;
    match command {
        UserSubcommand::Create(user) => {
            create_user(user);
        }
        UserSubcommand::Update(user) => {
            update_user(user);
        }
        UserSubcommand::Delete(delete_entity) => {
            delete_user(delete_entity);
        }
        UserSubcommand::Show => {
            show_users();
        }
    }
}

fn create_user(user: CreateUser) {
    println!("Creating user: {:?}", user);
    use crate::schema::users::dsl::*;

    let connection = establish_connection();
    let new_user = NewUser {
        name: &user.name,
        email: &user.email,
        removed: false,
    };

    diesel::insert_into(users)
        .values(&new_user)
        .execute(&connection)
        .expect("Error saving new user");
}

fn update_user(user: UpdateUser) {
    println!("Updating user: {:?}", user);
    use crate::schema::users::dsl::*;

    let connection = establish_connection();
    let db_user = DBUser {
        id: user.id,
        name: user.name,
        email: user.email,
        removed: false,
    };
    
    diesel::update(users.find(user.id))
        .set(&db_user)
        .execute(&connection)
        .expect("Error updating user");
}

fn delete_user(user: DeleteEntity) {
    println!("Deleting user: {:?}", user);
    use crate::schema::users::dsl::*;

    let connection = establish_connection();
    diesel::delete(users.find(user.id))
        .execute(&connection)
        .expect("Error deleting user");
}

fn show_users() {
    use crate::schema::users::dsl::*;

    let connection = establish_connection();
    let results = users
        .load::<DBUser>(&connection)
        .unwrap();

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{:?}", user);
    }
}