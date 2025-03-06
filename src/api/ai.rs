use crate::api::conversation::{Conversation, Message};
use serde_json::json;
use anyhow::{Result, Context};

pub struct DeepseekAi {
    api_key: String,
}

const DEEPSEEK_URL: &str = "https://api.deepseek.com/chat/completions";

fn extract_field(response_json: &serde_json::Value, field_path: &[&str]) -> Result<String> {
    let mut current = response_json;
    for &field in field_path {
        current = current.get(field).context(format!("Field {} not found", field))?;
    }
    if let Some(s) = current.as_str() {
        Ok(s.to_string())
    } else if let Some(n) = current.as_number() {
        Ok(n.to_string())
    } else {
        Err(anyhow::anyhow!("Field is not a string or number"))
    }
}

impl DeepseekAi {
    pub fn new(api_key: String) -> Self {
        DeepseekAi { api_key }
    }

    pub async fn chat(&self, conv: Conversation) -> Result<Message> {
        let client = reqwest::Client::new();
        let payload = json!({
            "messages": conv.to_messages(),
            "model": "deepseek-reasoner",
            "frequency_penalty": 0,
            "max_tokens": 2048,
            "presence_penalty": 0,
            "response_format": {
                "type": "text"
            }
        });
        let response = client
            .post(DEEPSEEK_URL)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&payload)
            .send()
            .await
            .context("Failed to send request to Deepseek API")?;

        let response_json = response
            .json::<serde_json::Value>()
            .await
            .context("Failed to parse response from Deepseek API")?;
        print!("{:?}", &response_json);

        let content = extract_field(&response_json["choices"][0], &["message", "content"]).unwrap();
        let reasoning = extract_field(&response_json["choices"][0], &["message", "reasoning_content"]).unwrap();
        let usage = extract_field(&response_json, &["usage", "completion_tokens"]).unwrap();

        Ok(Message::Assistant(format!("{}\n\nReasoning: {}\n\nUsage: {}", content, reasoning, usage)))
    }
}



mod test {
    #[cfg(test)]
    use crate::api;
    #[allow(dead_code)]
    const SAMPLE_JSON: &str = r#"{
        "choices": [{
            "finish_reason": "stop",
            "index": 0,
            "logprobs": null,
            "message": {
                    "content": "**9.8 is bigger than 9.11.**\n\n### Explanation:\n1. **Compare the whole numbers first:**  \n   Both numbers have the same whole number part (**9**), so we compare the decimals.\n\n2. **Equalize decimal places for clarity:**  \n   - \\( 9.8 = 9.80 \\) (adding a zero to match two decimal places).  \n   - Now compare **9.80** vs. **9.11**.\n\n3. **Compare tenths place:**  \n   - \\( 8 \\) (in 9.80) is greater than \\( 1 \\) (in 9.11).  \n   - **No need to check further digits** once a larger digit is found.\n\n### Why this works:\n- **Decimal places matter:**  \n  \\( 9.8 = 9 + \\frac{8}{10} \\), while \\( 9.11 = 9 + \\frac{11}{100} \\).  \n  Since \\( \\frac{8}{10} = \\frac{80}{100} \\), \\( 80/100 > 11/100 \\).\n\n### Common confusion:\n- If \\( 9.8 \\) were mistakenly written as \\( 9.08 \\), then \\( 9.11 \\) would be larger. But \\( 9.8 \\) is **not** \\( 9.08 \\).  \n\nLet me know if you'd like further clarification! 😊", 
                    "reasoning_content": "Okay, the user is asking which number is bigger between 9.11 and 9.8. Let me think about how to approach this.\n\nFirst, I know that comparing decimals can sometimes be tricky because of the different number of decimal places. The user might be confused by the two digits after the decimal in 9.11 versus the single digit in 9.8. \n\nI should start by explaining that to compare them, it's helpful to make sure both numbers have the same number of decimal places. So, 9.8 can be written as 9.80. That way, both numbers have two decimal places, making them easier to compare digit by digit.\n\nNext, compare the whole number parts. Both numbers have 9 as the whole number, so they are equal there. Then move to the tenths place: 1 in 9.11 versus 8 in 9.80. Since 1 is less than 8, that means 9.11 is actually smaller than 9.8.\n\nWait, but maybe the user is thinking of 9.8 as 9.08? That's a common mistake. I should address that possibility. If 9.8 were 9.08, then 9.11 would be larger. However, 9.8 is the same as 9.80, not 9.08. So, clarifying that point is important to avoid confusion.\n\nAlso, maybe the user is not familiar with decimal place values. So, breaking it down step by step would help. Emphasize that 0.8 is equivalent to 0.80, and comparing the tenths and hundredths places accordingly.\n\nAnother way to look at it is converting both numbers to fractions. 9.11 is 9 + 11/100, and 9.8 is 9 + 80/100. Comparing 11/100 and 80/100 shows clearly that 80/100 is larger, so 9.8 is bigger.\n\nI should also mention that sometimes people might misread 9.8 as 9.08, especially if they're not careful with decimal places. But in reality, 9.8 is nine and eight tenths, which is more than nine and eleven hundredths.\n\nSo putting it all together, the answer is that 9.8 is larger than 9.11. But the key is explaining the comparison clearly, addressing potential misunderstandings, and confirming why 9.8 is indeed the bigger number.",
                    "role": "assistant"
                    }
            }
        ], 
        "created": 1741251025, 
        "id": "9913e970-0d5d-4ac1-9620-438ce9edfa4e", 
        "model": "deepseek-reasoner", 
        "object": "chat.completion", 
        "system_fingerprint": "fp_5417b77867_prod0225", 
        "usage": {
            "completion_tokens": 814, 
            "completion_tokens_details": {"reasoning_tokens": 515}, 
            "prompt_cache_hit_tokens": 0, 
            "prompt_cache_miss_tokens": 43, 
            "prompt_tokens": 43,
            "prompt_tokens_details": {"cached_tokens": 0}, 
            "total_tokens": 857
            }
    }
            "#;

    #[tokio::test]
    async fn test_deepseek_ai() {
        use crate::api::setter::read_deepseek_api;
        use crate::api::conversation::{Conversation, Message};
        use crate::api::md_paraser::parse_markdown_file;
        
        // Get API key and initialize AI
        let api_key = read_deepseek_api();
        let ai = api::ai::DeepseekAi::new(api_key);
        
        // Parse conversation from file
        let conv: Conversation = parse_markdown_file("chat.md").unwrap();
        
        // Get response from Deepseek
        match ai.chat(conv).await {
            Ok(Message::Assistant(content)) => {
                println!("\nDeepseek Response Success:");
                println!("------------------------");
                println!("{}", content);
                println!("------------------------\n");
            }
            Ok(_) => println!("Received unexpected message type"),
            Err(e) => println!("Error getting response: {}", e),
        }
    }

    #[test]
    fn test_extract_field_content() {
        let response_json: serde_json::Value = serde_json::from_str(SAMPLE_JSON).unwrap();
        let content = api::ai::extract_field(&response_json["choices"][0], &["message", "content"]).unwrap();
        assert!(content.contains("9.8 is bigger than 9.11"));
        assert!(content.contains("Explanation:"));
    }

    #[test]
    fn test_extract_field_reasoning() {
        let response_json: serde_json::Value = serde_json::from_str(SAMPLE_JSON).unwrap();
        let reasoning = api::ai::extract_field(&response_json["choices"][0], &["message", "reasoning_content"]).unwrap();
        assert!(reasoning.contains("comparing decimals"));
        assert!(reasoning.contains("9.8 is larger than 9.11"));
    }

    #[test]
    fn test_extract_field_usage() {
        let response_json: serde_json::Value = serde_json::from_str(SAMPLE_JSON).unwrap();
        let usage = api::ai::extract_field(&response_json, &["usage", "completion_tokens"]).unwrap();
        assert_eq!(usage, "814");
    }

    #[test]
    fn test_extract_field_error_invalid_path() {
        let response_json: serde_json::Value = serde_json::from_str(SAMPLE_JSON).unwrap();
        let result = api::ai::extract_field(&response_json, &["invalid", "path"]);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Field invalid not found"));
    }

    #[test]
    fn test_extract_field_error_non_string() {
        let response_json: serde_json::Value = serde_json::from_str(SAMPLE_JSON).unwrap();
        let result = api::ai::extract_field(&response_json, &["choices"]);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Field is not a string or number"));
    }
}
