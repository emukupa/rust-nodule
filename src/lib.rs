// https://www.youtube.com/watch?v=969j0qnJGi8&t=167s

#![allow(dead_code, unused_variables)]

pub struct Credentials {
    username: String,
    password: String
}

enum Status {
    Connected,
    Interrupted,
}

fn connect_to_databases() -> Status{
    return Status::Connected;
}

fn get_user(){
    // get user from the database
}

fn login(creds: Credentials){
    // authenticate...
    get_user();
}

fn logout(){
    // log user out...
}

pub fn authenticate(creds: Credentials){
    if let Status::Connected = connect_to_databases() {
        login(creds);
    }
}

// generate using
// cargo modules generate tree
// cargo modules generate tree --with-types