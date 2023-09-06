use crate::type_sys::types::*;
use awc::Client;
use chrono;
use serde_derive::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct EmbedImage {
    url: String,
    proxy_url: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Embed {
    #[serde(rename = "type")]
    embed_type: String,
    description: String,
    image: EmbedImage,
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
    content_type: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Action {
    custom_id: String,
    label: String,
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
    trigger_id: String,
}

pub struct MidjourneyBot {
    user_request: PersonRequest,
    api_url: String,
    uuid: Uuid,
}

impl MidjourneyBot {
    pub fn new(user_request: PersonRequest, api_url: String, uuid: Uuid) -> MidjourneyBot {
        MidjourneyBot { user_request, api_url, uuid }
    }

    pub async fn imagine(&self) -> BotReceive {
        let mut resp: BotReceive = BotReceive {
            from: User { role: Role::Bot, uuid: Uuid::new_v4() },
            message: Message {
                message_type: MessageType::Text,
                content: "Empty imagine prompt".to_string(),
            },
            messaage_uuid: Uuid::new_v4(),
            timestamp: chrono::Local::now(),
        };

        let base_url: String = self.api_url.to_owned();
        let mut http_client: Client = awc::Client::default();
        let task_url: String = base_url + "/v1/api/trigger/imagine";

        let imagine_json = serde_json::json!({
            "prompt": self.user_request.message.content
        });

        let mj_response = http_client.post(task_url).send_json(&imagine_json).await;

        match mj_response {
            Ok(_) => resp.message.content = "Midjourney imagine request sent".to_string(),
            Err(_) => resp.message.content = "Failed to request Midjourney API".to_string(),
        }

        return resp;
    }
}

pub struct MidjourneyCallbackHandler {}
