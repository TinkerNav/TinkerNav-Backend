use chrono::Local;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    Image,
    Text,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub enum Role {
    Bot,
    Person,
    Tnav,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub enum Event {
    Start,
    Content,
    Done,
    Error,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub message_type: MessageType,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub role: Role,
    pub uuid: Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PersonRequest {
    pub to: User,
    pub message: Message,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BotReceive {
    pub from: User,
    pub message: Message,
    pub messaage_uuid: Uuid,
    pub timestamp: chrono::DateTime<Local>,
}

pub enum BotResponseType {
    Start,
    Content,
    Done,
    Error,
}

pub struct BotResponse {
    pub event: Event,
    pub message: Option<Message>,
    pub reply_message_uuid: Option<Uuid>,
    pub desc: Option<String>,
}

pub fn bot_response(
    bot_response_type: BotResponseType, message: Option<Message>, reply_message_uuid: Option<Uuid>,
    desc: Option<String>,
) -> BotResponse {
    match bot_response_type {
        BotResponseType::Start => {
            BotResponse { event: Event::Start, message: None, reply_message_uuid, desc: None }
        }
        BotResponseType::Content => {
            BotResponse { event: Event::Content, message, reply_message_uuid: None, desc: None }
        }
        BotResponseType::Done => {
            BotResponse { event: Event::Done, message: None, reply_message_uuid: None, desc: None }
        }
        BotResponseType::Error => {
            BotResponse { event: Event::Error, message: None, reply_message_uuid: None, desc }
        }
    }
}
