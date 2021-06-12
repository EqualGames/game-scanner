pub fn remove_quotes(text: &str) -> String {
    return String::from(text.to_string()).replace("\"", "");
}
