use clap::{Arg, Command};
use std::error::Error;
use std::io::{self, Write};

use llm_interface::{Message, Thread};
use open_router::OpenRouterClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("LLM CLI")
        .version("1.0")
        .author("Your Name")
        .about("Interact with an LLM model via CLI")
        .arg(
            Arg::new("model")
                .short('m')
                .long("model")
                .value_name("MODEL")
                .help("Sets the LLM model to use")
                .default_value("gpt-3.5-turbo")
                .required(false),
        )
        .get_matches();

    let model = matches.get_one::<String>("model").unwrap();
    let client = OpenRouterClient::from_env_variable(model)?;
    let mut thread = Thread::new();

    println!("Welcome to the LLM CLI. Type 'quit' to exit.");

    loop {
        print!("You: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.to_lowercase() == "quit" {
            break;
        }

        let user_message = Message::create_user_message(input);
        thread.append_message(user_message);

        match client.get_response(&thread).await {
            Ok((response, usage)) => {
                println!("AI: {}", response.content);
                if let Some(usage) = usage {
                    println!("Prompt tokens: {}", usage.prompt_tokens);
                    println!("Completion tokens: {}", usage.completion_tokens);
                    println!("Total tokens: {}", usage.total_tokens);
                }
                thread.append_message(response);
            },
            Err(e) => println!("Error: {}", e),
        }
    }

    println!("Thank you for using the LLM CLI. Goodbye!");
    Ok(())
}
