mod model;
use crate::TNStates;
use rocket::form::Form;
use rocket::*;

pub use model::Person;

use rocket::serde::{json::Json, Serialize};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PersonResponse {
    username: String,
    uuid: String,
}

#[derive(FromForm)]
pub struct PersonLoginForm {
    username: String,
    password: String,
}

#[post("/registry", data = "<form>")]
pub fn register(state: &State<TNStates>, form: Form<PersonLoginForm>) -> Json<PersonResponse> {
    // Register new user
    let connection = &mut state.pg_pool.get().unwrap();
    let username = &form.username;
    let password = &form.password;
    let new_user = model::Person::create(connection, username, password);
    Json(PersonResponse { username: new_user.username, uuid: new_user.uuid.to_string() })
}
