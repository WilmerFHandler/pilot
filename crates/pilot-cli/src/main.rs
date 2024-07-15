#![allow(dead_code, unused_variables)]

use std::io::Write;

use llm_interface::Message;
use open_router::OpenRouterClient;

#[tokio::main]
async fn main() {
    let client = OpenRouterClient::from_env_variable("meta-llama/llama-3-8b-instruct")
        .expect("Couldn't find api key. Make sure you have set your OPENROUTER_API_KEY enviroment variable!");
    let mut conversation = llm_interface::Conversation::new();
   
    loop {
        print!("You: ");
        let mut prompt = String::new();
        std::io::stdout().flush().expect("Failed to flush");
        std::io::stdin().read_line(&mut prompt).expect("Error");
        
        conversation.push(Message::create_user_message(prompt.as_str()));
        let (response, usage) = client.get_response(&conversation).await.expect("Failed to get response.");
        println!("Assistant: {}", response.content);
        conversation.push(response);
    }
}
