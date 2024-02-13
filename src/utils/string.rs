pub fn remove_quotes(text: &str) -> String {
    text.to_string().replace('"', "")
}
