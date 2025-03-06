# AI-CLI

A command line tool to chat with Deepseek AI. Conversations are presented in a markdown file. It is recommended to use it with VSCode or any other editor that provides markdown rendering and an integrated terminal.

## Features

- Set the Deepseek API key
- Create or use an existing markdown file for chat
- Get responses from Deepseek AI and save the conversation to the markdown file

## Usage

### Set the API Key

```sh
ai-cli key <api_key>
```

### Create or Use a Markdown File for Chat

```sh
ai-cli set chat.md
```

### Start a Chat Session

```sh
ai-cli chat
```

## Example

Here is an example of a conversation in a markdown file:

```md
# test.md

---
### System
---
You are a helper assistant

---
### User
---
9.8 and 9.11, which is bigger?

---
### Assistant
---
9.8 is larger than 9.11.  

**Explanation:**  
- **Decimal Place Comparison:**  
    - Both numbers have "9" in the units place.  
    - Compare the tenths place: **8** (in 9.8) vs. **1** (in 9.11). Since 8 > 1, 9.8 is larger.  

- **Equivalent Form:**  
    - Rewrite 9.8 as **9.80** to match decimal places. Now compare:  
        - **9.80** vs. **9.11** → 80 hundredths > 11 hundredths.  

- **Subtraction Check:**  
    - 9.80 - 9.11 = **0.69** (positive result confirms 9.8 > 9.11).  

**Answer:** 9.8 is greater than 9.11.
```

## Recommendation

- Use it in the VSCode terminal for the best experience.
