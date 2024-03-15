

use crate::core::ai_service::interface::*;
/*
use crate::core::ai_service::interface::ConversationData;
use crate::core::ai_service::interface::AIAssistant;
use crate::core::ai_service::interface::Roles;
 */


// This function has 3 steps: 
//1: 
fn get_gpt_response(conversation_data: &ConversationData, thread_id: i32, assistant: &dyn AIAssistant) -> String {
    // Retrieve the messages for the given thread_id
    /*let thread_messages = conversation_data.messages.get(&thread_id)
        .unwrap_or_else(|| panic!("Thread with ID {} not found", thread_id));
*/
    let thread_messages = conversation_data.messages.get(&thread_id);
    // Prepare the prompt by concatenating the messages
    let mut prompt = String::new();
    for message in thread_messages {
        match message.role {
            Roles::User => prompt.push_str(&format!("Human: {}\n", message.content)),
            Roles::Assistant => prompt.push_str(&format!("AI: {}\n", message.content)),
            Roles::System => prompt.push_str(&format!("System: {}\n", message.content)),
        }
    }

    // Call the AI assistant to generate a response
    assistant.generate_response(&prompt)
}
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