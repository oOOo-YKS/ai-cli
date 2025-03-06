use ai_cli::api::ai::DeepseekAi;
use ai_cli::api::setter::{ write_deepseek_api, check_file , read_resume_file, write_resume_file, read_deepseek_api};
use ai_cli::api::md_paraser::parse_markdown_file;
use std::env;
use std::io::Write;
use std::process;

const HELP_TEXT: &str = r#"
ai-cli - A CLI tool help you chat with Deepseek AI and save the conversation to a Markdown file

COMMANDS:
    key <api_key>           Set the Deepseek API key
    set <filename>          Create a new Markdown filet to chat in or use an existing one
    chat                    get a response from Deepseek AI and save the conversation to the Markdown file
    help                    Show this help message

EXAMPLES:
    ai-cli set chat.md
    ai-cli key <api_key>
    ai-cli chat

"#;

const SAMPLE_MARKDOWN: &str = r#"

---
### System
---
You are a helper assistant

---
### User
---
"#;

#[tokio::main]
async fn main() {
    check_file();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help_and_exit();
    }

    match args[1].as_str() {
        "set" => {
            if args.len() < 3 {
                eprintln!("Missing <chat_file> argument");
                print_help_and_exit();
            }
            create_markdown_file(&args[2]);
        }
        "key" => {
            if args.len() < 3 {
                eprintln!("Missing <api_key> argument");
                print_help_and_exit();
            }
            write_deepseek_api(&args[2]);
        }
        "chat" => {
            let api_key = read_deepseek_api();
            let ai = DeepseekAi::new(api_key);
            let filename = read_resume_file();
            let mut conversation = parse_markdown_file(&filename).unwrap();
            print!("Starting chat session with Deepseek AI...\n\n");
            print!("{}", conversation.to_markdown(filename.clone()));
            match ai.chat(conversation.clone()).await {
                Ok(ai_message) => {
                    conversation.add_message(ai_message.clone());
                    let markdown = conversation.to_markdown(filename.clone());
                    overwrite_markdown_file(&filename, markdown);
                }
                Err(e) => {
                    eprintln!("Error during chat session: {}", e);
                }
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
    let mut filename = filename.to_string();

    if !filename.ends_with(".md") {
        filename.push_str(".md");
    }

    use std::fs::OpenOptions;
    // if file exists, ignore create
    match OpenOptions::new()
        .write(true)
        .create_new(true) // create_new will ensure the file is created only if it does not exist
        .open(&filename)
    {
        Ok(mut file) => {
            let mut content = format!("# {}\n\n", filename);
            content.push_str(SAMPLE_MARKDOWN);
            if let Err(e) = file.write_all(content.as_bytes()) {
                eprintln!("Failed to write to file {}: {}", filename, e);
            } else {
                println!("File {} created successfully.", filename);
            }
        }
        Err(ref error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            println!("File {} found in this directory, please chat in this file", filename);
        }
        Err(e) => {
            eprintln!("Failed to create file {}: {}", filename, e);
        }
    }
    write_resume_file(&filename);
}

fn overwrite_markdown_file(filename: &str, content: String) {
    let mut filename = filename.to_string();

    if !filename.ends_with(".md") {
        filename.push_str(".md");
    }

    use std::fs::OpenOptions;
    // overwrite the file with content anyway
    match OpenOptions::new()
        .write(true)
        .truncate(true) // truncate will ensure the file is overwritten
        .open(&filename)
    {
        Ok(mut file) => {
            if let Err(e) = file.write_all(content.as_bytes()) {
                eprintln!("Failed to write to file {}: {}", filename, e);
            } else {
                println!("File {} overwritten successfully.", filename);
            }
        }
        Err(e) => {
            eprintln!("Failed to open file {}: {}", filename, e);
        }
    }
}
