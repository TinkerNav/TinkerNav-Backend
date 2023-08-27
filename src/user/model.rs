extern crate bcrypt;
use crate::schema::persons;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use uuid::Uuid;

use rocket::http::{Cookie, CookieJar};

#[derive(Queryable, Selectable, Debug, Insertable)]
#[diesel(table_name = crate::schema::persons)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Person {
    pub username: String,
    password_hash: String,
    pub uuid: Uuid,
}

impl Person {
    fn hash_password(password: &str) -> String {
        bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap()
    }

    fn verify_password(&self, password: &str) -> bool {
        let hash = &self.password_hash;
        bcrypt::verify(password, hash).unwrap()
    }

    pub fn find_username(conn: &mut PgConnection, username: &str) -> Person {
        persons::table
            .filter(persons::username.eq(username))
            .first(conn)
            .expect("Error getting user")
    }

    pub fn new(username: String, password_hash: String, uuid: Uuid) -> Person {
        Person { username, password_hash, uuid }
    }

    pub fn create(conn: &mut PgConnection, username: &str, password: &str) -> Person {
        let password_hash = Person::hash_password(password);
        let user_uuid = Uuid::new_v4();
        let username = username.to_string();
        let new_user =
            Person { username, password_hash: password_hash.to_string(), uuid: user_uuid };
        diesel::insert_into(persons::table)
            .values(&new_user)
            .get_result(conn)
            .expect("Error saving new user")
    }

    pub fn find(conn: &mut PgConnection, user_uuid: Uuid) -> Person {
        persons::table.find(user_uuid).first(conn).expect("Error getting user")
    }

    pub fn delete(conn: &mut PgConnection, user_uuid: Uuid) -> bool {
        diesel::delete(persons::table.find(user_uuid)).execute(conn).is_ok()
    }

    pub fn login(conn: &mut PgConnection, username: &str, password: &str) -> Person {
        let user = Person::find_username(conn, username);
        if !user.verify_password(password) {
            panic!("Wrong password");
        }
        user
    }
    
    pub fn current_user(conn: &mut PgConnection, cookies: &CookieJar<'_>) -> Person {
        let user_uuid = cookies
            .get_private("uuid")
            .expect("No uuid cookie found")
            .value()
            .parse::<Uuid>()
            .expect("Invalid uuid cookie");
        Person::find(conn, user_uuid)
    }
}


