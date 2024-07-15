#![allow(dead_code, unused_variables)]

use std::ops::{Index, IndexMut};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Role {
    User,
    Assistant,
    System,
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
    pub fn create_system_message(content: &str) -> Self {
        Message {
            role: Role::System,
            content: content.to_string(),
        }
    }
}

pub struct Conversation {
    pub messages: Vec<Message>,
}

// TODO: Maybe implement .iter() and .into_iter()?
impl Conversation {
    pub fn new() -> Self {
        Conversation {
            messages: Vec::new(),
        }
    }
    pub fn push(&mut self, message: Message) {
        self.messages.push(message);
    }
    pub fn insert(&mut self, index: usize, message: Message) {
        self.messages.insert(index, message);
    }
}
impl Index<usize> for Conversation {
    type Output = Message;
    fn index(&self, index: usize) -> &Self::Output {
        &self.messages[index]
    }
}
impl IndexMut<usize> for Conversation {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.messages[index]
    }
}

pub struct Usage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
}
