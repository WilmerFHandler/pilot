use llm_interface::{Message, Role};
use nestify::nest;
use serde::Serialize;


nest! {
    #[derive(Serialize)]*
    pub struct RequestBody {
        model: String,
        messages: Vec<struct SerializableMessage{
            role: String,
            content: String,
        }>,
    }
}

impl From<Message> for SerializableMessage {
    fn from(value: Message) -> Self {
        Self {
            role: match value.role {
                Role::User => "user".to_string(),
                Role::Assistant => "assistant".to_string(),
                Role::System => "system".to_string()
            },
            content: value.content,
        }
    }
}

impl RequestBody {
    pub fn new(model: String, messages: Vec<SerializableMessage>) -> Self {
        Self { model, messages }
    }

    // TODO: Move this into the FromIter trait
    pub fn from_messages(model: String, messages: &Vec<Message>) -> Self {
        Self {
            model,
            messages: messages
                .iter()
                .map(|m| SerializableMessage::from(m.clone()))
                .collect(),
        }
    }
}
