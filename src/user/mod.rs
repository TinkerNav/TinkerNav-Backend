mod model;
use rocket::*;
use crate::database;
use crate::TNStates;

pub use model::Person;

use rocket::serde::{Serialize, json::Json};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PersonResponse {
    username: String,
    uuid: String,
}


#[post("/registry")]
pub fn register(state: &State<TNStates>) -> Json<PersonResponse> {
    let connection = &mut database::establish_connection();
    let new_user = model::Person::create(connection, "test".to_string(), "test".to_string());
    Json(PersonResponse {
        username: new_user.username,
        uuid: new_user.uuid.to_string(),
    })
}


