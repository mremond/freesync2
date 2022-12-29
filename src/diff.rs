/// Compare content and handle cases where the file has already been moved before.
/// 
///     assert_eq!(content_diff("A", "A"), None);
///     assert_eq!(content_diff("A", "A\nB"), Some("A\nB".to_string()));
///     assert_eq!(content_diff("A\nB", "A"), Some("A\nB".to_string()));
///     assert_eq!(content_diff("A", "B"), Some("A\nB".to_string()));
fn content_diff(old: &str, new: &str) -> Option<String> {
    if old.contains(new) {
        println!("Content already exists, skipping.");
        None
    } else if new.contains(old) {
        println!("New content contains the old content, replacing.");
        Some(new.to_string())
    } else {
        println!("New and old are different. Appending new content to the old.");
        Some(format!("{}\n{}", old, new))
    }
}
pub trait Diffable {
    fn diff(self, other: &str) -> Option<String>;
}

impl Diffable for String {
    fn diff(self, other: &str) -> Option<String> {
        content_diff(&self, other)
    }
}

#[test]
fn test_content_skip() {
    let old = "Hello World";
    let new = "Hello World";
    assert_eq!(content_diff(old, new), None);
}

#[test]
fn test_extra_content() {
    let old = "Hello World";
    let new = "Hello World\nThis is a new line";
    assert_eq!(content_diff(old, new), Some(new.to_string()));
}

#[test]
fn test_appended_content() {
    let old = "Hello World";
    let new = "This is totally new content!";
    assert_eq!(content_diff(old, new), Some(format!("{}\n{}", old, new)));
}