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
    fn hash_password(password: &String) -> &String {
        password
    }

    pub fn new(username: String, password_hash: String, uuid: Uuid) -> Person {
        Person { username, password_hash, uuid }
    }

    pub fn create(conn: &mut PgConnection, username: String, password: String) -> Person {
        let password_hash = Person::hash_password(&password);
        let user_uuid = Uuid::new_v4();
        let new_user = Person { username, password_hash: password_hash.to_string(), uuid: user_uuid };
        diesel::insert_into(persons::table)
            .values(&new_user)
            .get_result(conn)
            .expect("Error saving new user")
    }

    pub fn get(conn: &mut PgConnection, user_uuid: Uuid) -> Person {
        persons::table.find(user_uuid).first(conn).expect("Error getting user")
    }

    pub fn delete(conn: &mut PgConnection, user_uuid: Uuid) -> bool {
        diesel::delete(persons::table.find(user_uuid)).execute(conn).is_ok()
    }
}
