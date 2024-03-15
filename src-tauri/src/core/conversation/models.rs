use std::collections::HashMap;

pub enum Roles {
    Assistant,
    User,
    System,
}
pub struct Message {
    pub id: i32,
    pub thread_id: i32,
    pub content: String,
    pub role: Roles,
    // Other message metadata fields
}

struct Thread {
    id: i32,
    conversation_id: i32,
    thread_participants: Vec<String>,
}
struct Conversation {
    id: i32,
    participants: Vec<String>,
}

pub struct ConversationData {
    pub conversations: HashMap<i32, Conversation>,
    pub threads: HashMap<i32, Vec<Thread>>,
    pub messages: HashMap<i32, Vec<Message>>,
}

pub struct ThreadData {
    pub thread_id: i32,
    pub messages: Vec<Message>,
}
