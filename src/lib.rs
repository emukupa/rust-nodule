// https://www.youtube.com/watch?v=969j0qnJGi8&t=167s

#![allow(dead_code, unused_variables)]

mod database;
mod auth_utils;

pub use auth_utils::models::Credentials;
use database::Status;

pub fn authenticate(creds: Credentials){
    if let Status::Connected = database::connect_to_databases() {
        auth_utils::login(creds);
    }
}

// generate using
// cargo modules generate tree
// cargo modules generate tree --with-types