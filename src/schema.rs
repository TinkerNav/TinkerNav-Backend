// @generated automatically by Diesel CLI.

diesel::table! {
    bot (uuid) {
        name -> Text,
        description -> Text,
        uuid -> Uuid,
    }
}

diesel::table! {
    bot_api_token (uuid) {
        uuid -> Text,
        bot_uuid -> Text,
        token -> Text,
    }
}

diesel::table! {
    person (uuid) {
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 512]
        password_hash -> Varchar,
        uuid -> Uuid,
    }
}

diesel::allow_tables_to_appear_in_same_query!(bot, bot_api_token, person,);
