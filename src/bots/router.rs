use rocket::serde::json::Json;
use serde::Deserialize;
use rocket::post;

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct TestJson {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[post("/test", format = "json", data = "<user_input>")]
pub fn test(user_input: Json<TestJson>) -> String {
    format!("print test {:?}", user_input)
}