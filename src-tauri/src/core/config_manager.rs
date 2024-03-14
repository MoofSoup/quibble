use std::collections::HashMap;

enum Roles {
    Assistant,
    User,
    System,
}
struct Message {
    id: i32,
    thread_id: i32,
    content: String,
    role: Roles,
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

struct ConversationData {
    conversations: HashMap<i32, Conversation>,
    threads: HashMap<i32, Vec<Thread>>,
    messages: HashMap<i32, Vec<Message>>,
}
// real code starts here

struct Prompts {
    gpt_prompt: String,
    claude_prompt: String,
    llama_prompt: String
}

enum SysPromptMode {
    GenericPrompt,
    TargetedPrompts(Prompts)
}
struct AssistantConfig {
    id: i32,
    name: String,
    sys_prompt_mode: SysPromptMode,

}
struct
struct ConfigData {
    assistant_config: AssistantConfig,

}
