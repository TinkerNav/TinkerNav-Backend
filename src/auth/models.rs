extern crate bcrypt;
use super::errors::{AuthError, AuthResult};
use crate::schema::person;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use uuid::Uuid;

pub trait User {
    fn get_uuid(&self) -> Uuid;
    fn get_name(&self) -> &str;
}

#[derive(Queryable, Selectable, Debug, Insertable)]
#[diesel(table_name = crate::schema::person)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Person {
    pub username: String,
    password_hash: String,
    uuid: Uuid,
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
        person::table.filter(person::username.eq(username)).first(conn).ok()
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
        diesel::insert_into(person::table)
            .values(&new_user)
            .get_result(conn)
            .map_err(|_| AuthError::CannotRegisterUser)
    }

    pub fn find(conn: &mut PgConnection, user_uuid: Uuid) -> Option<Person> {
        person::table.find(user_uuid).first(conn).ok()
    }

    pub fn delete(conn: &mut PgConnection, user_uuid: Uuid) -> bool {
        diesel::delete(person::table.find(user_uuid)).execute(conn).is_ok()
    }

    pub fn login(conn: &mut PgConnection, username: &str, password: &str) -> AuthResult<Person> {
        let user =
            Person::find_username(conn, username).ok_or(AuthError::InvalidUsernameOrPassword)?;
        if !user.verify_password(password)? {
            return Err(AuthError::InvalidUsernameOrPassword);
        }
        Ok(user)
    }
}

impl User for Person {
    fn get_uuid(&self) -> Uuid {
        self.uuid
    }

    fn get_name(&self) -> &str {
        self.username.as_str()
    }
}

#[derive(Queryable, Selectable, Debug, Insertable)]
#[diesel(table_name = crate::schema::bot)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Bot {
    pub name: String,
    pub description: String,
    uuid: Uuid,
}

impl Bot {
    pub fn new(name: String, description: String, uuid: Uuid) -> Bot {
        Bot { name, description, uuid }
    }

    pub fn create(conn: &mut PgConnection, name: &str, description: &str) -> AuthResult<Bot> {
        let bot_uuid = Uuid::new_v4();
        let name = name.to_string();
        let description = description.to_string();
        let new_bot = Bot { name, description, uuid: bot_uuid };
        diesel::insert_into(crate::schema::bot::table)
            .values(&new_bot)
            .get_result(conn)
            .map_err(|_| AuthError::CannotRegisterUser)
    }
    pub fn find(conn: &mut PgConnection, bot_uuid: Uuid) -> Option<Bot> {
        crate::schema::bot::table.find(bot_uuid).first(conn).ok()
    }
    pub fn delete(conn: &mut PgConnection, bot_uuid: Uuid) -> bool {
        diesel::delete(crate::schema::bot::table.find(bot_uuid)).execute(conn).is_ok()
    }
}

#[derive(Queryable, Selectable, Debug, Insertable)]
#[diesel(table_name = crate::schema::bot_api_token)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BotApiToken {
    pub token: String,
    pub uuid: Uuid,
    pub bot_uuid: Uuid,
}

impl BotApiToken {
    pub fn new(bot_uuid: Uuid, token: String, uuid: Uuid) -> BotApiToken {
        BotApiToken { bot_uuid, token, uuid }
    }

    pub fn create(conn: &mut PgConnection, bot_uuid: Uuid, token: &str) -> AuthResult<BotApiToken> {
        let bot_api_token_uuid = Uuid::new_v4();
        let token = token.to_string();
        let new_bot_api_token = BotApiToken { bot_uuid, token, uuid: bot_api_token_uuid };
        diesel::insert_into(crate::schema::bot_api_token::table)
            .values(&new_bot_api_token)
            .get_result(conn)
            .map_err(|_| AuthError::CannotRegisterUser)
    }
}
