use crate::context::Context;

pub fn create(
    draft: Option<String>,
    user_prompt: &str,
    context: &Context,
) -> String {
    let system_instructions = include_str!("INSTRUCTIONS.md");

    let mut parts = vec!["<commit_generation_request>".to_string()];

    // System Instructions Section (Non-negotiable)
    parts.push("    <system_instructions>".to_string());
    parts.push(system_instructions.to_string());
    parts.push("    </system_instructions>\n".to_string());

    // Context Section
    parts.push("    <context>".to_string());
    // Sandboxed user input
    // NOTE: CDATA is mainly used to treat the input as literal text
    parts.push(format!(
        "        <user_prompt><![CDATA[\n{}]]></user_prompt>",
        user_prompt
    ));
    if let Some(message) = draft {
        parts.push(format!(
            "        <draft_message>{}</draft_message>",
            message
        ));
    }
    parts.push(format!(
        "        <git_log><![CDATA[\n{}]]></git_log>",
        context.git_log
    ));
    parts.push(format!(
        "        <git_diff><![CDATA[\n{}]]></git_diff>",
        context.git_diff
    ));
    parts.push("    </context>".to_string());

    parts.push("</commit_generation_request>".to_string());

    parts.join("\n")
}
