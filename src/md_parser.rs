use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

use crate::conversation::{Message, Messages};

pub struct MdParser;

impl MdParser {
    pub fn parse_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<Message>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut messages: Vec<Message> = Vec::new();
        let mut current_section: Vec<Message> = Vec::new();
        let mut sys_message: Message = Message::System(String::new());
        let mut user_message: Message = Message::User(String::new());
        let mut ai_message: Message = Message::Agent(String::new());

        for line in reader.lines() {
            let line = line?;

            if line.starts_with("## AI") {
                if let Some(message) = current_section.pop() {
                    messages.push(message);
                }
                current_section.clear();
                ai_message = Message::Agent(String::new());
                current_section.push(ai_message);
            } else if line.starts_with("## User"){
                if let Some(message) = current_section.pop() {
                    messages.push(message);
                }
                current_section.clear();
                user_message = Message::User(String::new());
                current_section.push(user_message);
            } else if line.starts_with("## System") {
                if let Some(message) = current_section.pop() {
                    messages.push(message);
                }
                current_section.clear();
                sys_message = Message::System(String::new());
                current_section.push(sys_message);
            } else if let Some(message) = current_section.last_mut() {
                match message {
                    Message::System(content) => content.push_str(&format!("{}\n", line)),
                    Message::User(content) => content.push_str(&format!("{}\n", line)),
                    Message::Agent(content) => content.push_str(&format!("{}\n", line)),
                }
            }else if line.starts_with("# ") {
                // Skip main title
                continue;
            } else {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid section"));
            }
        }

        // Add last section
        if let Some(message) = current_section.pop() {
            messages.push(message);
        }
        Ok(messages)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parse_markdown() -> io::Result<()> {
        let temp_file = NamedTempFile::new()?;
        let test_content = r#"# Test Conversation
## System
System instructions here
## User-1
First user message
## AI-1
First AI response
## User-2
Second user message
## AI-2
Second AI response"#;

        write(temp_file.path(), test_content)?;

        let messages = MdParser::parse_file(temp_file.path())?;

        let mess = Messages::set_messages(messages.clone());
        print!("{:?}", mess.get_conversation());

        assert_eq!(messages.len(), 5);

        // Test system message
        if let Message::System(content) = &messages[0] {
            assert_eq!(content.trim(), "System instructions here");
        } else {
            panic!("Expected System message");
        }

        // Test first user message
        if let Message::User(content) = &messages[1] {
            assert_eq!(content.trim(), "First user message");
        } else {
            panic!("Expected User message");
        }

        // Test first AI message
        if let Message::Agent(content) = &messages[2] {
            assert_eq!(content.trim(), "First AI response");
        } else {
            panic!("Expected AI message");
        }

        // Test second user message
        if let Message::User(content) = &messages[3] {
            assert_eq!(content.trim(), "Second user message");
        } else {
            panic!("Expected User message");
        }

        // Test second AI message
        if let Message::Agent(content) = &messages[4] {
            assert_eq!(content.trim(), "Second AI response");
        } else {
            panic!("Expected AI message");
        }

        Ok(())
    }
}
