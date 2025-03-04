mod ai;
mod conversation;
mod md_parser;

use ai::DeepseekAi;
use anyhow;
use conversation::{Message, Messages};
use md_parser::{MdParser};
use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::process;

const HELP_TEXT: &str = "\
MD Creator - A tool for creating and managing Markdown files

USAGE:
    md-creator [COMMAND] [ARGS]

COMMANDS:
    create <filename>     Create a new Markdown file
    parse <filename>      Parse an existing Markdown file
    chat <filename>       Start a chat session and save to file
    help                 Show this help message

EXAMPLES:
    md-creator create notes.md
    md-creator parse conversation.md
    md-creator chat new_chat.md";

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help_and_exit();
    }

    match args[1].as_str() {
        "create" => {
            if args.len() != 3 {
                eprintln!("Usage: {} create <filename>", args[0]);
                process::exit(1);
            }
            create_markdown_file(&args[2]);
        }
        "chat" => {
            if args.len() != 3 {
                eprintln!("Usage: {} chat <filename>", args[0]);
                process::exit(1);
            }
            if let Err(e) = chat_session(&args[2]).await {
                eprintln!("Error in chat session: {}", e);
                process::exit(1);
            }
        }
        "help" | "-h" | "--help" => {
            print_help_and_exit();
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_help_and_exit();
        }
    }
}

fn print_help_and_exit() {
    println!("{}", HELP_TEXT);
    process::exit(0);
}

fn create_markdown_file(filename: &str) {
    if !filename.ends_with(".md") {
        let new_filename = filename.to_string() + ".md";
        match File::create(&new_filename) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error creating file: {}", e);
                process::exit(1);
            }
        };
        println!("✅ Created new Markdown file: {}", new_filename);
        return;
    }
    let mut file = match File::create(filename) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error creating file: {}", e);
            process::exit(1);
        }
    };

    // Write default template
    let template = format!("# {}\n\n## System\n\n", 
        filename.trim_end_matches(".md"));

    if let Err(e) = file.write_all(template.as_bytes()) {
        eprintln!("Error writing to file: {}", e);
        process::exit(1);
    }

    println!("✅ Created new Markdown file: {}", filename);
}

async fn chat_session(filename: &str) -> anyhow::Result<()> {
    let mut ai = DeepseekAi::new(
        "https://api.deepseek.com/chat/completions".to_string(),
        std::env::var("DEEPSEEK_API_KEY")
            .map_err(|_| anyhow::anyhow!("DEEPSEEK_API_KEY environment variable not set"))?,
    );

    println!("Starting chat session (type 'exit' to end)");
    let mut messages = Messages::new();
    messages.system("You are a helpful assistant.");
    ai.set_messages(messages);

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();
        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        println!();
        ai.user_message(input);
        ai.chat().await?;

        // Print AI's response
        if let Some(Message::Agent(response)) = ai.get_messages().get_conversation().last() {
            println!("🤖 {}\n", response);
        }
    }

    // Save conversation to file
    let mut file = File::create(filename)?;
    writeln!(file, "# Chat Session\n")?;
    writeln!(file, "## System\nYou are a helpful assistant.\n")?;

    let conversation = ai.get_messages().get_conversation();
    let mut message_count = 1;

    for message in conversation {
        match message {
            Message::System(content) => {
                writeln!(file, "## System\n{}\n", content)?;
            }
            Message::User(content) => {
                writeln!(file, "## User-{}\n{}\n", message_count, content)?;
                message_count += 1;
            }
            Message::Agent(content) => {
                writeln!(file, "## AI-{}\n{}\n", message_count - 1, content)?;
            }
        }
    }

    println!("Chat session saved to: {}", filename);
    Ok(())
}
