use std::collections::VecDeque;

use llm_interface::{Message, Role, Usage};
use nestify::nest;
use serde::Deserialize;

nest! {
    #[derive(Deserialize)]*
    pub struct ResponseBody {
        choices: Vec<struct Choice {
            finish_reason: String,
            message: pub struct SerializableResponseMessage {
                role: String,
                content: Option<String>,
            },
        }>,
        usage: Option<pub struct SerializableUsage {
            prompt_tokens: usize,
            completion_tokens: usize,
            total_tokens: usize,
        }>
    }
}

impl ResponseBody {
    pub fn try_extract_message(&mut self) -> Result<Message, String> {
        self.choices.pop()
            .ok_or_else(|| "No messages available".to_string())
            .and_then(|choice| Message::try_from(choice.message))
    }
    pub fn extract_usage(&mut self) -> Option<Usage> {
        self.usage.take().map(Usage::from)
    }
}

impl TryFrom<SerializableResponseMessage> for Message {
    type Error = String;
    fn try_from(value: SerializableResponseMessage) -> Result<Self, Self::Error> {
        Ok(Message {
            // TODO: See if there is a way to leverege the type system to enforce
            // all roles are covered
            role: match value.role.as_str() {
                "assistant" => Role::Assistant,
                "user" => Role::User,
                "system" => Role::System,
                _ => return Err(format!("Unknown role: {}", value.role)),
            },
            content: match value.content {
                Some(content) => content,
                None => return Err(String::from("Message must have content")),
            },
        })
    }
}

impl From<SerializableUsage> for Usage {
    fn from(value: SerializableUsage) -> Self {
        Self {
            prompt_tokens: value.prompt_tokens,
            completion_tokens: value.completion_tokens,
            total_tokens: value.total_tokens,
        }
    }
}

impl Clone for SerializableResponseMessage {
    fn clone(&self) -> Self {
        Self {
            role: self.role.clone(),
            content: self.content.clone(),
        }
    }
}
