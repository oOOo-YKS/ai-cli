use crate::conversation::Messages;
use serde_json::json;

pub struct DeepseekAi {
    base_url: String,
    api_key: String,
    messages: Messages,
}

impl DeepseekAi {
    pub fn new(base_url: String, api_key: String) -> Self {
        DeepseekAi {
            base_url,
            api_key,
            messages: Messages::new(),
        }
    }
    pub fn set_messages(&mut self, messages: Messages) {
        self.messages = messages;
    }

    pub fn user_message(&mut self, message: &str) {
        self.messages.user(message);
    }

    pub fn get_messages(&self) -> &Messages {
        &self.messages
    }

    pub async fn chat(&mut self) -> anyhow::Result<()> {
        let client = reqwest::Client::new();
        let response = client
            .post(&self.base_url)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&json!({
                "messages": self.messages.get_messages(),
                "model": "deepseek-reasoner",
                "frequency_penalty": 0,
                "max_tokens": 2048,
                "presence_penalty": 0,
                "response_format": {
                    "type": "text"
                }
            }))
            .send()
            .await?;

        let response_json = response.json::<serde_json::Value>().await?;

        if let Some(choices) = response_json.get("choices") {
            if let Some(first_choice) = choices.get(0) {
                if let Some(message) = first_choice.get("message") {
                    if let Some(content) = message.get("content").and_then(|c| c.as_str()) {
                        self.messages.agent(content);
                        return Ok(());
                    }
                }
            }
        }

        anyhow::bail!("Failed to parse response from Deepseek API")
    }
}
