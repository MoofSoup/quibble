

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