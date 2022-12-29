use crate::io;

#[derive(PartialEq, Debug, Clone)]
pub struct Note {
    title: String,
    content: String
}

impl Note {
    pub fn new(title: String, content: String) -> Note {
        Note{title: check_title_chars(&title), 
             content: replace_chars(&content)}
    }
    pub fn parse(content: &str) -> Option<Note> {
        let lines: Vec<&str> = content.lines().collect();
        if let Some(first_line) = lines.first() {
            // Match the pattern, markdown title then a blank line then some content.        
            if first_line.starts_with("# ") &&
            lines.len() > 2 &&
            lines[1].trim().is_empty() {
                let t = first_line[2..].to_string();
                println!("Found a title: {}", t);

                return Some(Note::new(
                    t,
                    lines[2..].join("\n").trim().to_string()
                ))
            }
        }
        None
    }
    pub fn write_file(self, path: &str) -> Option<String>{
        io::write_file(path, &self.title, &self.content)
    }
}

#[test]
fn test_blank_content() {
    assert_eq!(Note::parse(""), None);
}

#[test]
fn test_no_title() {
    assert_eq!(Note::parse("Hello World\n\nContent"), None);
}

#[test]
fn test_missing_blank_line() {
    assert_eq!(Note::parse("# My Title"), None);
    assert_eq!(Note::parse("# My Title\nContent"), None);
}

#[test]
fn test_header_two() {
    assert_eq!(Note::parse("## My Title\n\ncontent"), None);
}

#[test]
fn test_hash_tag() {
    assert_eq!(Note::parse("#My Title\n\ncontent"), None);
}

#[test]
fn test_correct_syntax() {
    assert_eq!(Note::parse("# My Title\n\nAnd my content."), 
               Some(Note::new("My Title".to_string(), "And my content.".to_string())));
}

/// Remove the characters that Obsidian doesn't like in the title.
/// 
///     assert_eq!(check_title_chars("Hello World"), "Hello World");
///     assert_eq!(check_title_chars("Hello*World"), "Hello_World");
/// 
fn check_title_chars(title: &str) -> String {
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
fn test_check_title_chars() {
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