use nanoid::nanoid;

/// Create a new id for a component.
pub fn create_id() -> String {
    format!("id-{id}", id = nanoid!())
}

/// Create a new id with suffix for a component.
pub fn create_suffix_id(suffix: &str) -> String {
    format!("id-{id}-{suffix}", id = nanoid!(), suffix = suffix)
}

/// create a standard div containing a component.
pub fn create_div(content: &str) -> String {
    format!(
        r#"<div class="w-100 h-100">{content}</div>"#,
        content = content
    )
}
