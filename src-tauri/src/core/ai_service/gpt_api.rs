

use crate::core::ai_service::interface::*;
// pasted from here <3
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize)]
struct ChatGPTRequest {
    messages: Vec<Message>,
}

#[derive(Deserialize)]
struct ChatGPTResponse {
    message: Message,
}

async fn extend_conversation(
    conversation_data: &mut ConversationData,
    thread_id: i32,
    api_key: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Extract messages from the specified thread
    let messages = conversation_data
        .messages
        .get(&thread_id)
        .cloned()
        .unwrap_or_default();

    // Serialize messages into JSON
    let request = ChatGPTRequest { messages };
    let json_data = serde_json::to_string(&request)?;

    // Send JSON to OpenAI's ChatGPT API using reqwest
    let client = Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .body(json_data)
        .send()
        .await?;

    // Check if the response is successful
    if response.status().is_success() {
        // Deserialize the JSON response
        let chatgpt_response: ChatGPTResponse = response.json().await?;

        // Update ConversationData with the new message
        let new_message = chatgpt_response.message;
        conversation_data
            .messages
            .entry(thread_id)
            .or_default()
            .push(new_message);

        Ok(())
    } else {
        // Handle API error
        let error_message = response.text().await?;
        Err(format!("API request failed: {}", error_message).into())
    }
}

// pasting from here <3

impl AIAssistant for GPTAssistant {
    fn generate_response(&self, prompt: &str) -> String {
        // Implementation for generating response using GPT
        // Use self.get_system_prompt() to get the appropriate system prompt
        // Make API calls to OpenAI's GPT model
        // Return the generated response as a String
        unimplemented!()
    }

    fn get_config(&self) -> &AssistantConfig {
        &self.config
    }

    fn set_config(&mut self, config: AssistantConfig) {
        self.config = config;
    }

    fn get_system_prompt(&self) -> String {
        match &self.config.sys_prompt_mode {
            SysPromptMode::GenericPrompt => "You are a helpful AI assistant.".to_string(),
            SysPromptMode::TargetedPrompts(prompts) => prompts.gpt_prompt.clone(),
        }
    }
}

struct GPTAssistant {
    config: AssistantConfig,
    // Other GPT-specific fields and methods
}