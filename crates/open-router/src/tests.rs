use tokio;
use crate::OpenRouterClient;
use llm_interface::{Message, Role};

#[tokio::test]
async fn test_openrouter_client() {
    let client = OpenRouterClient::from_env_variable("microsoft/phi-3-mini-128k-instruct:free")
        .expect(
            "Failed to create client. Make sure you have your openrouter api key as an env var",
        );
    let thread = crate::Thread {
        messages: vec![Message::create_user_message("Hello there my brother!")],
    };

    let (response, usage) = client
        .get_response(&thread)
        .await
        .expect("Error getting response");

    println!("User: {}", thread.messages[0].content);
    println!("Assistant: {}", response.content);

    assert_eq!(response.role, Role::Assistant);

    let usage = usage.unwrap();

    assert!(usage.total_tokens > 0);
    assert!(usage.total_tokens < 1000);
}
