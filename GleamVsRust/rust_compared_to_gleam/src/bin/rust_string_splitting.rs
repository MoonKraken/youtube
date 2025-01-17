fn process_message(input: &str) -> String {
    if input.starts_with("Assistant: ") {
        let result = &input["Assistant: ".len()..];
        result.to_string()
    } else if input.starts_with("User: ") {
        let result = &input["User: ".len()..];
        let reversed_result: String = result.chars().rev().collect();
        reversed_result
    } else if input.starts_with("System: ") {
        "**System message redacted**".to_string()
    } else {
        "malformed line".to_string()
    }
}
