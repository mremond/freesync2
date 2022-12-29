#[derive(PartialEq, Debug)]
pub struct Note {
    pub title: String,
    pub content: String
}

pub fn find_content_title(content: &str) -> Option<Note> {
    let lines: Vec<&str> = content.lines().collect();
    if let Some(first_line) = lines.first() {
        // Match the pattern, markdown title then a blank line then some content.        
        if first_line.starts_with("# ") &&
           lines.len() > 2 &&
           lines[1].trim().is_empty() {
            let t = first_line[2..].to_string();
            println!("Found a title: {}", t);

            return Some(Note{                
                title: t,
                content: lines[2..].join("\n").trim().to_string()
            })
        }
    }
    None
}

#[test]
fn test_blank_content() {
    assert_eq!(find_content_title(""), None);
}

#[test]
fn test_no_title() {
    assert_eq!(find_content_title("Hello World\n\nContent"), None);
}

#[test]
fn test_missing_blank_line() {
    assert_eq!(find_content_title("# My Title"), None);
    assert_eq!(find_content_title("# My Title\nContent"), None);
}

#[test]
fn test_header_two() {
    assert_eq!(find_content_title("## My Title\n\ncontent"), None);
}

#[test]
fn test_hash_tag() {
    assert_eq!(find_content_title("#My Title\n\ncontent"), None);
}

#[test]
fn test_correct_syntax() {
    assert_eq!(find_content_title("# My Title\n\nAnd my content."), Some(Note{title: "My Title".to_string(), content: "And my content.".to_string()}));
}

/// Compare content and handle cases where the file has already been moved before.
/// 
///     assert_eq!(content_diff("A", "A"), None);
///     assert_eq!(content_diff("A", "A\nB"), Some("A\nB".to_string()));
///     assert_eq!(content_diff("A\nB", "A"), Some("A\nB".to_string()));
///     assert_eq!(content_diff("A", "B"), Some("A\nB".to_string()));
pub fn content_diff(old: &str, new: &str) -> Option<String> {
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

/// Remove the characters that Obsidian doesn't like in the title.
/// 
///     assert_eq!(check_title_chars("Hello World"), "Hello World");
///     assert_eq!(check_title_chars("Hello*World"), "Hello_World");
/// 
pub fn check_title_chars(title: &str) -> String {
    title.replace("*", "_")
         .replace("\"", "_")
         .replace("\\", "_")
         .replace("/", "_")
         .replace("<", "_")
         .replace("<", "_")
         .replace(">", "_")
         .replace(":", "_")
         .replace("|", "_")
         .replace("?", "_")
}

#[test]
pub fn test_check_title_chars() {
    // A couple strings that it should not change
    assert_eq!(check_title_chars("Hello World"), "Hello World");
    assert_eq!(check_title_chars("Hello_World"), "Hello_World");
    assert_eq!(check_title_chars("Hello-World"), "Hello-World");

    // All the strings that it should change.
    assert_eq!(check_title_chars("Hello*World"), "Hello_World");
    assert_eq!(check_title_chars("Hello\"World"), "Hello_World");
    assert_eq!(check_title_chars("Hello\\World"), "Hello_World");
    assert_eq!(check_title_chars("Hello/World"), "Hello_World");
    assert_eq!(check_title_chars("Hello<World"), "Hello_World");
    assert_eq!(check_title_chars("Hello>World"), "Hello_World");
    assert_eq!(check_title_chars("Hello:World"), "Hello_World");
    assert_eq!(check_title_chars("Hello|World"), "Hello_World");
    assert_eq!(check_title_chars("Hello?World"), "Hello_World");
}

// Replace characters that are hard to type on the freewrite.
pub fn replace_chars(contents: &str) -> String {
    contents.replace("---", "—")
            .replace("--", "–")
}

#[test]
fn test_replace_chars() {
    assert_eq!(replace_chars("hyphen-stays"), "hyphen-stays");
    assert_eq!(replace_chars("en--dash"), "en–dash");
    assert_eq!(replace_chars("em---dash"), "em—dash");
}