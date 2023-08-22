use crate::schema::users;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable, Debug, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub username: String,
    password_hash: String,
    pub uuid: Uuid,
}

impl User {
    fn hash_password(password: &String) -> &String {
        password
    }

    pub fn new(username: String, password_hash: String, uuid: Uuid) -> User {
        User { username, password_hash, uuid }
    }

    pub fn create(conn: &mut PgConnection, username: String, password: String) -> User {
        let password_hash = User::hash_password(&password);
        let user_uuid = Uuid::new_v4();
        let new_user = User { username, password_hash: password_hash.to_string(), uuid: user_uuid };
        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(conn)
            .expect("Error saving new user")
    }

    pub fn get(conn: &mut PgConnection, user_uuid: Uuid) -> User {
        users::table.find(user_uuid).first(conn).expect("Error getting user")
    }

    pub fn delete(conn: &mut PgConnection, user_uuid: Uuid) -> bool {
        diesel::delete(users::table.find(user_uuid)).execute(conn).is_ok()
    }
}
