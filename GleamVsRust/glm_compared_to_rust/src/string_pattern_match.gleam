import gleam/string

fn handle_input(input: String) -> String {
  case input {
    "Assistant: " <> dialog -> dialog
    "System: " <> dialog -> "**System Message Redacted**"
    "User: " <> dialog -> string.reverse(dialog)
    _ -> "malformed line"
  }
}
