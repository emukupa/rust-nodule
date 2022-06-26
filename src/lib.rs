#![allow(dead_code, unused_variables)]

struct Credentials {
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

fn authenticate(creds: Credentials){
    if let Status::Connected = connect_to_databases() {
        login(creds);
    }
}