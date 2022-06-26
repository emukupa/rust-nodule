pub enum Status {
    Connected,
    Interrupted,
}

pub fn connect_to_databases() -> Status{
    return Status::Connected;
}

pub fn get_user(){
    // get user from the database
}