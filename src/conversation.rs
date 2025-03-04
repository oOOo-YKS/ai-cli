use serde_json::json;

#[derive(Debug, Clone)]
pub enum Message {
    System(String),
    User(String),
    Agent(String),
}

impl Message {
    fn to_json_message(&self) -> serde_json::Value {
        match self {
            Message::System(content) => json!({
                "role": "system",
                "content": content
            }),
            Message::User(content) => json!({
                "role": "user",
                "content": content
            }),
            Message::Agent(content) => json!({
                "role": "assistant",
                "content": content
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Messages {
    messages: Vec<Message>,
}

impl Messages {
    pub fn new() -> Self {
        Messages {
            messages: Vec::new(),
        }
    }

    pub fn set_messages(messages: Vec<Message>) -> Self{
        Messages {
            messages,
        }
    }

    pub fn system(&mut self, message: &str) {
        self.messages.push(Message::System(message.to_string()));
    }

    pub fn user(&mut self, message: &str) {
        self.messages.push(Message::User(message.to_string()));
    }

    pub fn agent(&mut self, message: &str) {
        self.messages.push(Message::Agent(message.to_string()));
    }

    pub fn get_messages(&self) -> Vec<serde_json::Value> {
        self.messages.iter().map(|m| m.to_json_message()).collect()
    }

    pub fn get_conversation(&self) -> &[Message] {
        &self.messages
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_messages() {
        let mut messages = Messages::new();
        messages.system("Hello");
        messages.user("Hi");
        messages.agent("Hello");

        let messages_json = messages.get_messages();
        assert_eq!(messages_json.len(), 3);
        assert_eq!(
            messages_json[0],
            json!({
                "role": "system",
                "content": "Hello"
            })
        );
        assert_eq!(
            messages_json[1],
            json!({
                "role": "user",
                "content": "Hi"
            })
        );
        assert_eq!(
            messages_json[2],
            json!({
                "role": "assistant",
                "content": "Hello"
            })
        );
    }
}
