// @generated automatically by Diesel CLI.

diesel::table! {
    persons (uuid) {
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 512]
        password_hash -> Varchar,
        uuid -> Uuid,
    }
}
