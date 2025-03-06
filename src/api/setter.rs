use std::fs;
use std::path::Path;

pub fn read_deepseek_api() -> String {
    let file_path = "setting.ai";
    match fs::read_to_string(file_path) {
        Ok(contents) => {
            for line in contents.lines() {
                let parts: Vec<&str> = line.split('=').collect();
                if parts.len() != 2 {
                    continue;
                }
                if parts[0] == "DEEPSEEK_API" {
                    return parts[1].to_string();
                }
            }
            String::new()
        },
        Err(e) => {
            println!("Failed to read file: {}", e);
            String::new()
        }
    }
}

pub fn write_deepseek_api(api_key: &str) {
    let file_path = "setting.ai";
    let mut new_contents = String::new();
    let mut found = false;

    match fs::read_to_string(file_path) {
        Ok(contents) => {
            for line in contents.lines() {
                let parts: Vec<&str> = line.split('=').collect();
                if parts.len() == 2 && parts[0] == "DEEPSEEK_API" {
                    new_contents.push_str(&format!("DEEPSEEK_API={}\n", api_key));
                    found = true;
                } else {
                    new_contents.push_str(line);
                    new_contents.push('\n');
                }
            }
            if !found {
                new_contents.push_str(&format!("DEEPSEEK_API={}\n", api_key));
            }
        },
        Err(_) => {
            new_contents.push_str(&format!("DEEPSEEK_API={}\n", api_key));
        }
    }

    match fs::write(file_path, new_contents) {
        Ok(_) => println!("DEEPSEEK_API updated successfully."),
        Err(e) => println!("Failed to update DEEPSEEK_API: {}", e),
    }
}

pub fn write_resume_file(resume_file: &str) {
    let file_path = "setting.ai";
    let mut new_contents = String::new();
    let mut found = false;

    match fs::read_to_string(file_path) {
        Ok(contents) => {
            for line in contents.lines() {
                let parts: Vec<&str> = line.split('=').collect();
                if parts.len() == 2 && parts[0] == "RESUME_FILE" {
                    new_contents.push_str(&format!("RESUME_FILE={}\n", resume_file));
                    found = true;
                } else {
                    new_contents.push_str(line);
                    new_contents.push('\n');
                }
            }
            if !found {
                new_contents.push_str(&format!("RESUME_FILE={}\n", resume_file));
            }
        },
        Err(_) => {
            new_contents.push_str(&format!("RESUME_FILE={}\n", resume_file));
        }
    }

    println!("{}", new_contents);
    match fs::write(file_path, new_contents) {
        Ok(_) => println!("RESUME_FILE updated successfully."),
        Err(e) => println!("Failed to update RESUME_FILE: {}", e),
    }
}

pub fn read_resume_file() -> String {
    let file_path = "setting.ai";
    match fs::read_to_string(file_path) {
        Ok(contents) => {
            for line in contents.lines() {
                let parts: Vec<&str> = line.split('=').collect();
                if parts.len() != 2 {
                    continue;
                }
                if parts[0] == "RESUME_FILE" {
                    return parts[1].to_string();
                }
            }
            String::new()
        },
        Err(e) => {
            println!("Failed to read file: {}", e);
            String::new()
        }
    }
}

pub fn check_file() {
    let file_path = "setting.ai";
    let file_content = "DEEPSEEK_API=To_BE_FILLED_BY_PROGRAM\nRESUME_FILE=To_BE_FILLED_BY_PROGRAM\n";
    if !Path::new(file_path).exists() {
        match fs::write(file_path, file_content) {
            Ok(_) => println!("File 'setting.ai' created successfully."),
            Err(e) => println!("Failed to create file: {}", e),
        }
    } else {
        println!("File 'setting.ai' already exists.");
    }
}

mod test {
    use std::ptr::eq;

    use super::*;

    #[test]
    fn test_check_file() {
        check_file();
        write_deepseek_api("test_key");
        let api_key = read_deepseek_api();
        assert_eq!(api_key, "test_key");
        write_deepseek_api("test_key_1");
        let api_key = read_deepseek_api();
        assert_eq!(api_key, "test_key_1");
        write_resume_file("sample.md_2");
        let resume_file = read_resume_file();
        assert_eq!(resume_file, "sample.md_2");
    }
}