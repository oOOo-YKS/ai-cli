use serde_json::json;

#[derive(Debug, Clone)]
pub enum Message {
    System(String),
    User(String),
    Assistant(String),
}

#[derive(Debug, Clone)]
pub struct Conversation {
    messages: Vec<Message>,
}

impl Conversation {
    fn get_top_message(&self) -> Option<&Message> {
        self.messages.last()
    }

    pub fn auto_add(&mut self, message: String) {
        // if the last message is a user message, add the assistant message
        // else add the user message
        match self.get_top_message() {
            Some(Message::User(_)) => {
                self.add_message(Message::Assistant(message));
            }
            _ => {
                self.add_message(Message::User(message));
            }
        }
    }
    
    pub fn add_message(&mut self, message: Message) {
        match message {
            Message::User(_) => {
                if let Some(Message::Assistant(_)) | Some(Message::System(_)) = self.get_top_message() {
                    self.messages.push(message);
                } else {
                    panic!("User message must follow an Assistant or System message");
                }
            }
            Message::Assistant(_) => {
                if let Some(Message::User(_)) = self.get_top_message() {
                    self.messages.push(message);
                } else {
                    panic!("Assistant message must follow a User message");
                }
            }
            Message::System(_) => {
                panic!("System message can only be the first message");
            }
        }
    }

    pub fn system(&mut self, message: String) -> Result<(), String> {
        let system_message = Message::System(message);
        self.add_message(system_message);
        Ok(())
    }

    pub fn assistant(&mut self, message: String) -> Result<(), String> {
        let assistant_message = Message::Assistant(message);
        self.add_message(assistant_message);
        Ok(())
    }

    pub fn user(&mut self, message: String) -> Result<(), String> {
        let user_message = Message::User(message);
        self.add_message(user_message);
        Ok(())
    }

    pub fn new(sys_message: Message) -> Conversation {
        Conversation {
            messages: vec![sys_message],
        }
    }

    pub fn to_messages(&self) -> Vec<serde_json::Value> {
        self.messages.iter().map(|msg| {
            match msg {
                Message::System(text) => json!({
                    "content": text,
                    "role": "system"
                }),
                Message::User(text) => json!({
                    "content": text,
                    "role": "user"
                }),
                Message::Assistant(text) => json!({
                    "content": text,
                    "role": "assistant"
                })
            }
        }).collect()
    }

    pub fn to_markdown(&self, filename:String) -> String {
        let mut markdown = String::new();
        markdown.push_str(format!("# {}\n\n", filename).as_str());
        for message in &self.messages {
            match message {
                Message::System(text) => markdown.push_str(&format!("---\n### System\n---\n{}\n\n", text)),
                Message::User(text) => markdown.push_str(&format!("---\n### User\n---\n{}\n\n", text)),
                Message::Assistant(text) => markdown.push_str(&format!("---\n### Assistant\n---\n{}\n\n", text)),
            }
        }
        markdown
    }
}

mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_conversation() {
        let sys_message = Message::System("You are a helpful assistant".to_string());
        let conversation = Conversation::new(sys_message);
        assert_eq!(conversation.messages.len(), 1);
    }

    #[test]
    fn test_conversation_messages() {
        let sys_message = Message::System("You are a helpful assistant".to_string());
        let conversation = Conversation::new(sys_message);
        let messages = conversation.to_messages();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0]["role"], "system");
        assert_eq!(messages[0]["content"], "You are a helpful assistant");
    }
}
