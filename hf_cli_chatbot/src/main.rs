use openai_api_rs::v1::{api::OpenAIClient, chat_completion::{ChatCompletionMessage, ChatCompletionRequest, Content, MessageRole}};
use std::io::{self, Write};
use io::{stdin, stdout};

#[tokio::main]
pub async fn main() {
    let base_url = std::env::var("HF_BASE_URL").unwrap_or("https://router.huggingface.co/v1".to_string());
    let token = std::env::var("HUGGINGFACE_TOKEN").expect("HUGGINGFACE_TOKEN not set");
    let mut client = OpenAIClient::builder()
        .with_endpoint(base_url)
        .with_api_key(token)
        .build()
        .expect("could not create OpenAI client");

    let model = "Qwen/Qwen3-235B-A22B-Instruct-2507:cerebras";
    
    let mut messages: Vec<ChatCompletionMessage> = vec![];
    loop {
        print!("> ");
        stdout().flush().unwrap(); // Ensure prompt is displayed before input
        let mut user_input = String::new();
        stdin().read_line(&mut user_input).expect("Failed to read input");
        let user_input = user_input.trim(); // Remove trailing newline
        messages.push(ChatCompletionMessage {
            role: MessageRole::user,
            content: Content::Text(user_input.to_string()),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        });

        let req = ChatCompletionRequest::new(model.to_string(), messages.clone());
        // Send the request and handle response
        match client.chat_completion(req).await {
            Ok(response) => {

                if let Some(choice) = response.choices.first() {
                    if let Some(content) = choice.message.content.as_ref() {
                        println!("{}", content);
                        messages.push(ChatCompletionMessage {
                            role: MessageRole::assistant,
                            content: Content::Text(content.to_string()),
                            name: None,
                            tool_calls: None,
                            tool_call_id: None,
                        });
                    } else {
                        println!("No content received");
                    }
                } else {
                    println!("No response received");
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}