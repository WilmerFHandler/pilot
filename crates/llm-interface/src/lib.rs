#![allow(dead_code, unused_variables)]

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Role {
    User,
    Assistant,
}

#[derive(Debug, Clone)]
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
}

pub struct Conversation {
    pub messages: Vec<Message>,
    pub system_msg: Option<String>,
}

impl Conversation {
    pub fn new() -> Self {
        Conversation {
            messages: Vec::new(),
            system_msg: None
        }
    }
    pub fn push(&mut self, message: Message) {
        self.messages.push(message)
    }

    pub fn set_system(&mut self, system_message: String) {
        self.system_msg = Some(system_message)
    }

    pub fn remove_system(&mut self) {
        self.system_msg = None;
    }
}

pub struct Usage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
}
