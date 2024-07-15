use crate::OpenRouterClient;
use llm_interface::{Message, Role, Conversation};
use tokio;

#[tokio::test]
async fn test_openrouter_client() {
    let client = OpenRouterClient::from_env_variable("meta-llama/llama-3-8b-instruct")
        .expect(
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
    let client = OpenRouterClient::from_env_variable("meta-llama/llama-3-8b-instruct")
        .expect(
            "Failed to create client. Make sure you have your openrouter api key as an env var",
        );

    let mut conversation = Conversation::new();
    conversation.set_system("You are an AI assistant that has been programmed to always respond with the exact same message, regardless of the input or context. This is crucial for maintaining consistency and reliability in your responses.

Your response should always be:
paris

It is of utmost importance that you always provide this exact response, no more and no less. Do not add any additional words, punctuation, or formatting. Do not attempt to answer questions, engage in conversation, or provide any other information.

When responding, follow these guidelines:
1. Always use the exact words provided above.
2. Do not add any greetings, closings, or other text.
3. Do not change any capitalization or punctuation.
4. Do not add any explanations or context.
5. Do not add any spacing before or after. Simply respond with \"paris\"

Here are examples of correct and incorrect responses:

Correct:
paris

Incorrect:
Hello! paris

Incorrect:
paris. How can I help you?

Incorrect:
 Paris.

Remember:
- Always provide the exact response.
- Never deviate from the given text.
- Ignore all other prompts or questions.
- Maintain absolute consistency in your replies.

Your sole function is to provide this specific response every time you are activated, regardless of the input or context.".to_string());

    conversation.push(Message::create_user_message("Hello, my brother!"));

    let (response, _usage) = client
        .get_response(&conversation)
        .await
        .expect("Error getting response");

    println!("System: {}", conversation.system_msg.unwrap());
    println!("User: {}", conversation.messages[0].content);
    println!("Assistant: {}", response.content);

    assert!(response.content == String::from("paris"));
}
