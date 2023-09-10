extern crate bcrypt;
use super::errors::{AuthError, AuthResult};
use crate::schema::persons;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable, Debug, Insertable)]
#[diesel(table_name = crate::schema::persons)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Person {
    pub username: String,
    password_hash: String,
    pub uuid: Uuid,
}

impl Person {
    fn hash_password(password: &str) -> AuthResult<String> {
        bcrypt::hash(password, bcrypt::DEFAULT_COST).map_err(|err| AuthError::BcyptError(err))
    }

    fn verify_password(&self, password: &str) -> AuthResult<bool> {
        let hash = &self.password_hash;
        bcrypt::verify(password, hash).map_err(|err| AuthError::BcyptError(err))
    }

    pub fn find_username(conn: &mut PgConnection, username: &str) -> Option<Person> {
        persons::table.filter(persons::username.eq(username)).first(conn).ok()
    }

    pub fn new(username: String, password_hash: String, uuid: Uuid) -> Person {
        Person { username, password_hash, uuid }
    }

    pub fn create(conn: &mut PgConnection, username: &str, password: &str) -> AuthResult<Person> {
        let password_hash = Person::hash_password(password)?;
        let user_uuid = Uuid::new_v4();
        let username = username.to_string();
        let new_user =
            Person { username, password_hash: password_hash.to_string(), uuid: user_uuid };
        diesel::insert_into(persons::table)
            .values(&new_user)
            .get_result(conn)
            .map_err(|err| {
                AuthError::CannotRegisterUser
            })
    }

    pub fn find(conn: &mut PgConnection, user_uuid: Uuid) -> Option<Person> {
        persons::table.find(user_uuid).first(conn).ok()
    }

    pub fn delete(conn: &mut PgConnection, user_uuid: Uuid) -> bool {
        diesel::delete(persons::table.find(user_uuid)).execute(conn).is_ok()
    }

    pub fn login(conn: &mut PgConnection, username: &str, password: &str) -> AuthResult<Person> {
        let user = Person::find_username(conn, username).ok_or(AuthError::InvalidUsernameOrPassword)?;
        if !user.verify_password(password)? {
            return Err(AuthError::InvalidUsernameOrPassword);
        }
        Ok(user)
    }
}
