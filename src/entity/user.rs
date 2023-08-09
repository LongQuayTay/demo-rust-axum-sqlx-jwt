use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct User {
    id: u64,
    username: String,
    password: String,
    name: String,
    age: Option<u8>
}

impl User {
    pub fn new(id: u64, username: String, password: String, name: String, age: Option<u8>) -> Self {
        Self { id, username, password, name, age }
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_username(&self) -> String {
        self.username.to_string()
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn get_password(&self) -> String {
        self.password.to_string()
    }

    pub fn get_age(&self) -> Option<u8> {
        self.age
    }
}
