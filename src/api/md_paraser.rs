use std::fs::File;
use std::io::{self, Read};
use crate::api::conversation::{Conversation, Message};

#[derive(Debug, Clone, PartialEq, Eq)]
enum ReadingType {
    System,
    User,
    Assistant, 
    NotReading,
}

// struct to hold the state of the markdown scanner
#[derive(Debug)]
struct MarkdownScanner {
    buffer: String,
    messages: Vec<Message>,
    reading_type: ReadingType,
    reading_lines: Vec<String>,
}

impl MarkdownScanner {
    // Create a new MarkdownScanner with initial state
    fn new() -> MarkdownScanner {
        MarkdownScanner {
            buffer: String::new(),
            messages: Vec::new(),
            reading_type: ReadingType::NotReading,
            reading_lines: vec!["".to_string(), "".to_string(), "".to_string()],
        }
    }

    // Scan a line of markdown and update the state accordingly
    fn scan(&mut self, line: &str) {
        // Update the reading lines buffer
        self.reading_lines = vec![self.reading_lines[1].clone(), self.reading_lines[2].clone(), line.to_string()];
        
        // Check for section delimiters
        if self.reading_lines[0].starts_with("---") && self.reading_lines[2].starts_with("---") {
            match self.reading_lines[1].as_str() {
                "### System" => {
                    self.buffer.clear();
                    self.reading_type = ReadingType::System;
                }
                "### User" => {
                    // delete the last two lines of the buffer string
                    let mut lines: Vec<&str> = self.buffer.lines().collect();
                    if lines.len() >= 2 {
                        lines.pop();
                        lines.pop();
                    }
                    self.buffer = lines.join("\n");

                    self.push_message();
                    self.reading_type = ReadingType::User;
                }
                "### Assistant" => {
                    // delete the last two lines of the buffer string
                    let mut lines: Vec<&str> = self.buffer.lines().collect();
                    if lines.len() >= 2 {
                        lines.pop();
                        lines.pop();
                    }
                    self.buffer = lines.join("\n");

                    self.push_message();
                    self.reading_type = ReadingType::Assistant;
                }
                _ => {}
            }
        } else if self.reading_type != ReadingType::NotReading {
            // Append the line to the buffer if currently reading a section
            self.buffer.push_str(line);
            self.buffer.push('\n');
        }
    }

    // Push the current buffer as a message and clear the buffer
    fn push_message(&mut self) {
        // Add the buffer content to messages based on the current reading type
        match self.reading_type {
            ReadingType::System => self.messages.push(Message::System(self.buffer.clone())),
            ReadingType::User => self.messages.push(Message::User(self.buffer.clone())),
            ReadingType::Assistant => self.messages.push(Message::Assistant(self.buffer.clone())),
            ReadingType::NotReading => {}
        }
        self.buffer.clear();
    }

    // Finalize the scanning process by pushing any remaining buffer content as a message
    fn finalize(&mut self) {
        self.push_message();
    }

    fn to_conversation(&self) -> Conversation {
        let mut result = Conversation::new(self.messages[0].clone());
        for message in &self.messages[1..] {
            match message {
                Message::System(text) => result.system(text.clone()).unwrap(),
                Message::User(text) | Message::Assistant(text) => result.auto_add(text.clone()),
            }
        }
        result
    }
}

pub fn parse_markdown_file(file_path: &str) -> Result<Conversation, io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut scanner = MarkdownScanner::new();
    for line in contents.lines() {
        scanner.scan(line);
    }
    scanner.finalize();
    Ok(scanner.to_conversation())
}

mod test {
    #[test]
    fn test_joining_lines() {
        let string_1 = "Hello";
        let string_2 = "World";
        let string_3 = "!";
        let strings = vec![string_1, string_2, string_3];
        let new_string = strings.join("\n");
        print!("{}", new_string);
    }

    #[allow(dead_code)]
    const TEST_MARKDOWN: &str = r#"
# sample

---
### System
---
You are a helper assistant

---
### User
---
hi
how are you today

---
### Assistant
---
how can i help you today
"#;

    #[test]
    fn test_markdown_scanner() {
        let mut scanner = super::MarkdownScanner::new();
        for line in TEST_MARKDOWN.lines() {
            scanner.scan(line);
        }
        scanner.finalize();
        print!("{:?}", scanner.messages);
        assert_eq!(scanner.messages.len(), 3);
    }

    #[test]
    fn test_parse_markdown_file() {
        let conversation = super::parse_markdown_file("chat.md").unwrap();
        let messages = conversation.to_messages();
        print!("{:?}", messages);
        assert!(!messages.is_empty());
    }
}
