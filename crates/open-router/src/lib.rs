#![allow(dead_code)]

#[cfg(test)]
mod tests;

mod request_body;
use request_body::RequestBody;

mod response_body;
use response_body::ResponseBody;


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
        let api_key = std::env::var("OPENROUTER_API_KEY")?;

        Ok(OpenRouterClient {
            api_key,
            model: model.to_string(),
        })
    }

    pub async fn get_response(
        &self,
        conversation: &Conversation,
    ) -> Result<(Message, Option<Usage>), Box<dyn Error>> {
        let client = reqwest::Client::new();

        let body = RequestBody::from_messages(self.model.clone(), &conversation.messages);

        let response = client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await?;

        let mut response_body: ResponseBody = response.json().await?;
        Ok((
            response_body.try_extract_message()?,
            response_body.extract_usage(),
        ))
    }
}
