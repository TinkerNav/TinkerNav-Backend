use rocket::post;
use rocket::serde::json::Json;
use serde::Deserialize;
use crate::type_sys::types::PersonRequest;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct EmbedImage {
    url: String,
    proxy_url: String
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Embed {
    #[serde(rename = "type")]
    embed_type: String,
    description: String,
    image: EmbedImage
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Attachment {
    filename: String,
    id: i128,
    proxy_url: String, 
    size: i64,
    url: String,
    spoiler: bool,
    height: i32,
    width: i32,
    content_type: String
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Action {
    custom_id: String,
    label: String
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct MidjourneyCallbackData {
    #[serde(rename = "type")]
    event_type: String,
    id: i128,
    content: String,
    attachments: Vec<Attachment>,
    actions: Vec<Action>,
    embeds: Vec<Embed>,
    trigger_id: String
}

#[post("/midjourney", format = "json", data = "<user_request>")]
pub fn midjourney(user_request: Json<PersonRequest>) -> String {
    format!("print test {:?}", user_request)
}

#[post("/midjourney_callback", format = "json", data = "<callback_data>")]
pub fn midjourney_callback(callback_data: Json<MidjourneyCallbackData>) -> String {
    format!("callback received {:?}", callback_data)
}