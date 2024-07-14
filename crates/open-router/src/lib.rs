#![allow(dead_code)]

#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};
use std::error::Error;

use llm_interface::*;

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
        let body = RequestBody {
            // HACK: Maybe these don't need to be cloned
            model: self.model.clone(),
            messages: thread.messages.clone(),
        };

        let response = client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await?;

        let response_body: ResponseBody = response.json().await?;
        Ok((
            response_body.choices[0].message.clone(),
            Some(response_body.usage),
        ))
    }
}

#[derive(Serialize)]
struct RequestBody {
    model: String,
    messages: Vec<Message>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
    finish_reason: String,
}

#[derive(Deserialize)]
struct ResponseBody {
    choices: Vec<Choice>,
    usage: Usage,
}
