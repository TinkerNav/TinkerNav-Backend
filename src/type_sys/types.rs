use chrono::Local;
use uuid::Uuid;
use chrono;
use serde::Deserialize;
use serde::Serialize;

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
enum MessageType {
    Image,
    Text
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
enum Role {
    Bot,
    Person,
    Tnav
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
enum Event {
    Start,
    Content,
    Done,
    Error
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    message_type: MessageType,
    content: String
} 

#[derive(Serialize, Deserialize)]
pub struct User {
    role: Role,
    uuid: Uuid
}

#[derive(Serialize, Deserialize)]
pub enum MessageProtocol {
    PersonRequest {
        to: User,
        message: Message,
    },

    BotReceive {
        from: User,
        message: Message,
        message_uuid: Uuid,
        timestamp: chrono::DateTime<Local>
    }
}

pub enum BotResponseType {
    Start,
    Content,
    Done,
    Error
}

pub struct BotResponse {
    event: Event,
    message: Option<Message>,
    reply_message_uuid: Option<Uuid>,
    desc: Option<String>
}

pub fn bot_response(
    bot_response_type: BotResponseType, message: Option<Message>, 
    reply_message_uuid: Option<Uuid>, desc: Option<String>) -> BotResponse{
    match bot_response_type {
        BotResponseType::Start => BotResponse { 
            event: Event::Start, message: None, 
            reply_message_uuid: reply_message_uuid, desc: None 
        },
        BotResponseType::Content => BotResponse { 
            event: Event::Content, message: message,
            reply_message_uuid: None, desc: None
        },
        BotResponseType::Done => BotResponse { 
            event: Event::Done, message: None, 
            reply_message_uuid: None, desc: None 
        },
        BotResponseType::Error => BotResponse { 
            event: Event::Error, message: None, 
            reply_message_uuid: None, desc: desc
        }
    }
}