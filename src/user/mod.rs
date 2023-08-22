pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(id: i32, name: String, email: String, password: String) -> User {
        User {
            id,
            name,
            email,
            password,
        }
    }
}


