#![allow(dead_code, unused_variables)]

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

impl Message {
    pub fn create_user_message(content: &str) -> Self {
        Message {
            role: Role::User,
            content: content.to_string(),
        }
    }
    pub fn create_assistant_message(content: &str) -> Self {
        Message {
            role: Role::Assistant,
            content: content.to_string(),
        }
    }
    pub fn create_system_message(content: &str) -> Self {
        Message {
            role: Role::System,
            content: content.to_string(),
        }
    }
}

pub struct Thread {
    pub messages: Vec<Message>,
}

impl Thread {
    pub fn new() -> Self {
        Thread {
            messages: Vec::new()
        }
    }

    pub fn push(&mut self, message: Message) {
        self.messages.push(message)
    }
}

#[derive(Deserialize)]
pub struct Usage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
}
