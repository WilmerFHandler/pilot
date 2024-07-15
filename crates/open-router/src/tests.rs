use std::fs;

use crate::OpenRouterClient;
use llm_interface::{Conversation, Message, Role};
use tokio;

#[tokio::test]
async fn test_openrouter_client() {
    let client = OpenRouterClient::from_env_variable("meta-llama/llama-3-8b-instruct").expect(
        "Failed to create client. Make sure you have your openrouter api key as an env var",
    );

    let mut conversation = Conversation::new();
    conversation.push(Message::create_user_message("Hello, my brother!"));

    let (response, usage) = client
        .get_response(&conversation)
        .await
        .expect("Error getting response");

    println!("User: {}", conversation.messages[0].content);
    println!("Assistant: {}", response.content);

    assert_eq!(response.role, Role::Assistant);

    let usage = usage.unwrap();

    assert!(usage.total_tokens > 0);
    assert!(usage.total_tokens < 1000);
}

#[tokio::test]
async fn test_system_message() {
    let client = OpenRouterClient::from_env_variable("meta-llama/llama-3-8b-instruct").expect(
        "Failed to create client. Make sure you have your openrouter api key as an env var",
    );

    let mut conversation = Conversation::new();
    let system_message = Message::create_system_message(
        fs::read_to_string("test_system_message.txt")
            .expect("Failed to read system message")
            .as_str(),
    );
    conversation.push(system_message);
    conversation.push(Message::create_user_message("Hello, my brother!"));

    let (response, _usage) = client
        .get_response(&conversation)
        .await
        .expect("Error getting response");

    println!("System: {}", conversation.messages[0].content);
    println!("User: {}", conversation.messages[1].content);
    println!("Assistant: {}", response.content);

    assert!(response.content == String::from("paris"));
}
