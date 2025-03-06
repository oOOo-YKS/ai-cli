#[derive(Debug)]
pub enum Message {
    System(String),
    User(String),
    Assistant(String),
}

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

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        result.push_str(&format!("{{\"messages\":["));
        for message in &self.messages {
            match message {
                Message::System(text) => result.push_str(&format!("{{\"content\":\"{}\",\"role\":\"system\"}}", text)),
                Message::User(text) => result.push_str(&format!(",{{\"content\":\"{}\",\"role\":\"user\"}}", text)),
                Message::Assistant(text) => result.push_str(&format!(",{{\"content\":\"{}\",\"role\":\"assistant\"}}", text)),
            }
        }
        result.push_str("]}");
        result
    }
}

mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_conversation() {
        let sys_message = Message::System("Your are a helpful assistant".to_string());
        let conversation = Conversation::new(sys_message);
        assert_eq!(conversation.messages.len(), 1);
    }

    #[test]
    fn test_conversation_to_string() {
        let sys_message = Message::System("Your are a helpful assistant".to_string());
        let conversation = Conversation::new(sys_message);
        println!("{}", conversation.to_string());
        assert_eq!(conversation.to_string(), "{\"messages\":[{\"content\":\"Your are a helpful assistant\",\"role\":\"system\"}]}");
    }
}