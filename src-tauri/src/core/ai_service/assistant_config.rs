// config.rs contains the business logic for interacting with the config data model.
// Assistant configuration is handled as a part of the AiAssistant trait

struct TargetedPrompts {
    gpt_prompt: String,
    claude_prompt: String,
    llama_prompt: String
}

enum SysPromptMode {
    GenericPrompt,
    TargetedPrompt(TargetedPrompts)
}
struct AssistantConfig {
    id: i32,
    name: String,
    sys_prompt_mode: SysPromptMode,

}