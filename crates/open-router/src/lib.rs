#![allow(dead_code)]

#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};
use std::error::Error;

use llm_interface::*;
use nestify::nest;

nest! {
    #[derive(Serialize)]*
    struct RequestBody {
        model: String,
        messages: Vec<struct SerializableMessage{
            role: String,
            content: String,
        }>,
    }
}

impl RequestBody {
    // FIX: Should model be &str or Strings?
    fn new(model: &str, messages: &Vec<Message>) -> Self {
        Self {
            model: model.to_string(),
            messages: messages
                .iter()
                .map(|m| SerializableMessage {
                    role: match m.role {
                        Role::User => "user".to_string(),
                        Role::Assistant => "assistant".to_string(),
                    },
                    // FIX: Could this clone be removed
                    content: m.content.clone(),
                })
                .collect(),
        }
    }
}

nest! {
    #[derive(Deserialize)]*
    struct ResponseBody {
        choices: Vec<struct Choice {
            finish_reason: String,
            message: struct SerializableResponseMessage {
                role: String,
                content: Option<String>,
            },
        }>,
        usage: Option<struct SerializableUsage {
            prompt_tokens: usize,
            completion_tokens: usize,
            total_tokens: usize,
        }>
    }
}

impl TryFrom<SerializableResponseMessage> for Message {
    type Error = String;
    fn try_from(value: SerializableResponseMessage) -> Result<Self, Self::Error> {
        Ok(Message {
            role: match value.role.as_str() {
                "assistant" => Role::Assistant,
                "user" => Role::User,
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

pub struct OpenRouterClient {
    api_key: String,
    model: String,
}

impl OpenRouterClient {
    pub fn new(api_key: String, model: &str) -> Self {
        OpenRouterClient {
            api_key,
            model: model.to_string(),
        }
    }

    pub fn from_env_variable(model: &str) -> Result<Self, Box<dyn Error>> {
        let api_key = std::env::var("OPENROUTER_API_KEY");

        Ok(OpenRouterClient {
            api_key: api_key?,
            model: model.to_string(),
        })
    }

    pub async fn get_response(
        &self,
        thread: &Thread,
    ) -> Result<(Message, Option<Usage>), Box<dyn Error>> {
        let client = reqwest::Client::new();
        let body = RequestBody::new(&self.model, &thread.messages);

        let response = client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await?;

        let mut response_body: ResponseBody = response.json().await?;
        Ok((
            Message::try_from(response_body.choices.remove(0).message)?,
            response_body.usage.map(Usage::from),
        ))
    }
}
